use crate::wireguard::{WireguardDevice, WireguardDeviceWithPrivateKey};
use std::sync::LazyLock;
const WIREGUARD_KEY_PATH: &str = "../wireguard/wg0.pub";
const WIREGUARD_DEVICES_PATH: &str = "../wireguard/peers.json";

// a lazy static to load the wireguard public key from the file system.
// but use the rust standard library instead of lazy static crate
pub static WIREGUARD_KEY: LazyLock<String> = LazyLock::new(|| {
    std::fs::read_to_string(WIREGUARD_KEY_PATH)
        .expect("Failed to read Wireguard public key")
        .trim()
        .to_string()
});

impl WireguardDevice {
    pub fn load_all() -> std::io::Result<Vec<WireguardDevice>> {
        let file = std::fs::read(WIREGUARD_DEVICES_PATH)?;
        let devices = serde_json::from_slice::<Vec<WireguardDevice>>(&file)?;
        Ok(devices)
    }

    pub fn save_all(devices: &Vec<WireguardDevice>) -> std::io::Result<()> {
        let file = serde_json::to_string_pretty(devices)?;
        std::fs::write(WIREGUARD_DEVICES_PATH, file)?;
        Ok(())
    }

    pub fn add_device(name: String) -> dioxus::Result<WireguardDeviceWithPrivateKey> {
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
}

pub fn to_wireguard_config(
    WireguardDeviceWithPrivateKey {
        device,
        private_key,
    }: &WireguardDeviceWithPrivateKey,
) -> String {
    let server_public_key = &*WIREGUARD_KEY;

    let id = device.id;
    format!(
        "[Interface]
Address = 10.90.90.{id}/24, fd06:f100:1796::{id}/64
DNS = 1.1.1.1
PrivateKey = {private_key}

[Peer]
AllowedIPs = 0.0.0.0/0, ::/0
Endpoint = stefnotch.duckdns.org:3478
PublicKey = {server_public_key}
",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_wireguard() {
        let devices = WireguardDevice::load_all();

        dbg!(devices);
    }
}
