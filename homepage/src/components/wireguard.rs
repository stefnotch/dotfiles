use dioxus::prelude::*;

use crate::{components::QrCode, wireguard::WireguardDevice};

#[component]
pub fn Wireguard() -> Element {
    // use_signal is a hook. Hooks in dioxus must be run in a consistent order every time the component is rendered.
    // That means they can't be run inside othetr hooks, async blocks, if statements, or loops.
    //
    // use_signal is a hook that creates a state for the component. It takes a closure that returns the initial value of the state.
    // The state is automatically tracked and will rerun any other hooks or components that read it whenever it changes.

    let my_ip = use_resource(move || async move { get_my_ip().await });

    rsx! {
        "You are"
        if let Some(response) = &*my_ip.read() {
            match response {
                Ok(ip) => rsx! {
                    p { "Your IP: {ip}" }
                },
                Err(err) => rsx! { "Failed to fetch IP: {err}" },
            }
        } else {
            "Loading..."
        }

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

    let mut device = use_signal::<RequestState<String, String>>(|| RequestState::Idle);

    let display_device = match device() {
        RequestState::Idle => rsx! {},
        RequestState::Loading => rsx! {
            p { "Loading..." }
        },
        RequestState::Error(err) => rsx! {
            p { "{err}" }
        },
        RequestState::Success(config) => {
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

#[post("/api/vpn/add_device", ip: SimpleClientIp)]
async fn add_vpn_device(name: String) -> Result<String> {
    use crate::wireguard::{WireguardConfig, server::to_wireguard_config};
    let mut config = WireguardConfig::load()?;

    let group = ip.0.and_then(|ip| {
        Some(
            config
                .peers
                .iter()
                .find(|device| device.name == name)?
                .group
                .clone(),
        )
    });

    let device = config.add_device(name, group)?;
    Ok(to_wireguard_config(&device))
}

#[get("/api/vpn/devices")]
async fn get_vpn_devices() -> dioxus::Result<Vec<WireguardDevice>> {
    use crate::wireguard::WireguardConfig;
    let config = WireguardConfig::load()?;
    let devices = config
        .peers
        .into_iter()
        // .filter(|device| device.group == "vpn")
        .collect();
    Ok(devices)
}

#[get("/api/my_ip", ip: SimpleClientIp)]
async fn get_my_ip() -> dioxus::Result<String> {
    let my_ip =
        ip.0.map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());
    Ok(my_ip)
}

// Recursive expansion of define_extractor! macro
// ===============================================

#[derive(Debug, Clone, Copy)]
pub struct SimpleClientIp(pub Option<std::net::IpAddr>);

#[cfg(feature = "server")]
impl<S> dioxus_server::axum::extract::FromRequestParts<S> for SimpleClientIp
where
    S: Sync,
{
    type Rejection = axum_client_ip::Rejection;
    async fn from_request_parts(
        parts: &mut dioxus_server::axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(
            axum_client_ip::RightmostXForwardedFor::from_request_parts(parts, _state)
                .await
                .map_or_else(|_| SimpleClientIp(None), |ip| SimpleClientIp(Some(ip.0))),
        )
    }
}
