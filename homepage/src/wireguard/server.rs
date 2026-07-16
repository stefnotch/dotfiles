use crate::wireguard::WireguardDevice;

const WIREGUARD_DEVICES_PATH: &str = "../wireguard/peers.json";

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
