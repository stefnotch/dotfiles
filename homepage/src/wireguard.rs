#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct WireguardDevice {
    pub name: String,
    pub group: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub id: u32,
}

const WIREGUARD_DEVICES_PATH: &str = "../wireguard/peers.json";

impl WireguardDevice {
    pub fn load_all() -> Vec<WireguardDevice> {
        let file = std::fs::read(WIREGUARD_DEVICES_PATH).unwrap();
        let devices = serde_json::from_slice::<Vec<WireguardDevice>>(&file).unwrap();
        devices
    }

    pub fn save_all(devices: &Vec<WireguardDevice>) {
        let file = serde_json::to_string_pretty(devices).unwrap();
        std::fs::write(WIREGUARD_DEVICES_PATH, file).unwrap();
    }
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
