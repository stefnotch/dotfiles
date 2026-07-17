use dioxus::logger::tracing;

use crate::wireguard::WireguardConfig;
use crate::wireguard::{WireguardDevice, WireguardDeviceWithPrivateKey};
use std::sync::LazyLock;

const WIREGUARD_PATH: &str = "../wireguard";
fn get_wireguard_path(file: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(
        std::env::var("WIREGUARD_PATH").unwrap_or_else(|_| WIREGUARD_PATH.to_owned()),
    )
    .join(file)
}

pub static WIREGUARD_KEY: LazyLock<String> = LazyLock::new(|| {
    std::fs::read_to_string(get_wireguard_path("wg0.pub"))
        .expect("Failed to read Wireguard public key")
        .trim()
        .to_string()
});

impl WireguardConfig {
    pub fn load() -> std::io::Result<Self> {
        let file = std::fs::read(get_wireguard_path("config.json"))?;
        let config = serde_json::from_slice::<Self>(&file)?;
        Ok(config)
    }

    pub fn save(&self) -> std::io::Result<()> {
        let file = serde_json::to_string_pretty(self)?;
        std::fs::write(get_wireguard_path("config.json"), file)?;
        Ok(())
    }

    pub fn add_device(
        &mut self,
        name: String,
        group: Option<String>,
    ) -> dioxus::Result<WireguardDeviceWithPrivateKey> {
        tracing::info!("Adding Wireguard device: {}", name);
        let (private_key, public_key) = generate_wireguard_keys()?;

        let id = self.next_available_id()?;

        let device = WireguardDevice {
            name,
            group: group.unwrap_or_else(|| "guest".to_owned()),
            public_key,
            id,
        };

        self.peers.push(device.clone());
        self.save()?;

        // Taken from https://serverfault.com/a/1110966
        // Add peer to Wireguard, it will get routed to wg0.
        // The entire wireguard subnet is routed there.
        std::process::Command::new("wg")
            .args([
                "set",
                "wg0",
                "peer",
                &device.public_key,
                "allowed-ips",
                &format!("{}.{id}/32,{}::{id}/128", self.prefix.ipv4, self.prefix.ipv6),
            ])
            .status()?;

        Ok(WireguardDeviceWithPrivateKey {
            device,
            private_key,
        })
    }

    fn next_available_id(&self) -> dioxus::Result<u32> {
        // 1 is used by the server itself
        (2..255u32)
            .find(|id| !self.peers.iter().any(|device| device.id == *id))
            .ok_or_else(|| dioxus::CapturedError::msg("No available ID for new Wireguard device"))
    }
}

fn generate_wireguard_keys() -> dioxus::Result<(String, String)> {
    use std::io::Write as _;
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
    Ok((private_key, public_key))
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
        let config = WireguardConfig::load().expect("Failed to load Wireguard config");

        dbg!(config);
    }
}
