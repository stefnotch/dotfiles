use dioxus::prelude::*;

use crate::{
    components::QrCode,
    wireguard::{WireguardDevice, WireguardDeviceWithPrivateKey},
};

#[component]
pub fn Wireguard() -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside other hooks, async blocks, if statements, or loops.
    //
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.

    rsx! {
        "Add a computer to Wireguard!"
        br {}
        AddDevice {}
    }
}

#[derive(Clone)]
enum RequestState<T, E> {
    Idle,
    Loading,
    Error(E),
    Success(T),
}

#[component]
pub fn AddDevice() -> Element {
    let mut device_name = use_signal(|| String::new());

    let mut device =
        use_signal::<RequestState<WireguardDeviceWithPrivateKey, String>>(|| RequestState::Idle);

    let display_device = match device() {
        RequestState::Idle => rsx! {},
        RequestState::Loading => rsx! {
            p { "Loading..." }
        },
        RequestState::Error(err) => rsx! {
            p { "{err}" }
        },
        RequestState::Success(new_device) => {
            let config = to_wireguard_config(&new_device);
            rsx! {
                QrCode { data: config.as_bytes().to_vec() }
                br {}
                DownloadTextFile {
                    text: config.clone(),
                    filename: format!("{}.conf", device_name()),
                    "Download config"
                }
            }
        }
    };

    rsx! {
        input {
            placeholder: "Device Name",
            oninput: move |evt| {
                device_name.set(evt.value());
            },
        }
        button {
            style: "margin-left: 10px; width: 3rem; font-size: 2rem; line-height: 2rem;",
            onclick: move |_| async move {
                if let RequestState::Loading = *device.read() {
                    return;
                }
                device.set(RequestState::Loading);
                let response = add_vpn_device(device_name()).await;
                match response {
                    Ok(new_device) => {
                        device.set(RequestState::Success(new_device));
                    }
                    Err(err) => {
                        device.set(RequestState::Error(err.to_string()));
                    }
                }
            },
            disabled: device_name().len() < 3 || matches!(*device.read(), RequestState::Loading),
            "+"
        }
        br {}
        {display_device}
    }
}

#[component]
fn DownloadTextFile(text: String, filename: String, children: Element) -> Element {
    let encoded = percent_encoding::utf8_percent_encode(&text, percent_encoding::NON_ALPHANUMERIC)
        .to_string();
    let href = format!("data:text/plain,{encoded}");
    rsx! {
        a {
            href: "{href}",
            download: "{filename}",
            {children}
        }
    }
}

#[post("/api/vpn/add_device")]
async fn add_vpn_device(name: String) -> Result<WireguardDeviceWithPrivateKey> {
    use std::io::Write as _;
    // call `wg genkey` to generate a new private key
    let private_key = std::process::Command::new("wg")
        .arg("genkey")
        .output()?
        .stdout;

    let mut public_key_command = std::process::Command::new("wg")
        .arg("pubkey")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = public_key_command
        .stdin
        .take()
        .expect("Failed to open stdin");

    let output = std::thread::scope(|s| {
        s.spawn(|| {
            stdin
                .write_all(&private_key)
                .expect("Failed to write to stdin");
            // Close stdin to signal that we're done writing
            drop(stdin);
        });
        public_key_command.wait_with_output()
    })?;

    let private_key = String::from_utf8(private_key)
        .expect("failed to convert to string")
        .trim()
        .to_owned();
    let public_key = String::from_utf8(output.stdout)?.trim().to_owned();

    let mut all_devices = WireguardDevice::load_all()?;
    // 1 is used by the server itself
    let id = (2..255u32)
        .find(|id| !all_devices.iter().any(|device| device.id == *id))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "No available ID for new Wireguard device",
            )
        })?;

    let device = WireguardDevice {
        name,
        group: "vpn".to_string(),
        public_key,
        id,
    };

    all_devices.push(device.clone());
    WireguardDevice::save_all(&all_devices)?;

    Ok(WireguardDeviceWithPrivateKey {
        device,
        private_key,
    })
}

fn to_wireguard_config(
    WireguardDeviceWithPrivateKey {
        device,
        private_key,
    }: &WireguardDeviceWithPrivateKey,
) -> String {
    const SERVER_PUBLIC_KEY: &str = include_str!("../../../wireguard/wg0.pub");

    let id = device.id;
    format!(
        "[Interface]
Address = 10.90.90.{id}/24, fd06:f100:1796::{id}/64
DNS = 1.1.1.1
PrivateKey = {private_key}

[Peer]
AllowedIPs = 0.0.0.0/0, ::/0
Endpoint = stefnotch.duckdns.org:3478
PublicKey = {SERVER_PUBLIC_KEY}
",
    )
}

#[get("/api/vpn/devices")]
async fn get_vpn_devices() -> dioxus::Result<Vec<WireguardDevice>> {
    let devices = WireguardDevice::load_all()?
        .into_iter()
        // .filter(|device| device.group == "vpn")
        .collect();
    Ok(devices)
}
