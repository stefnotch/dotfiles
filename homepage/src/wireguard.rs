use dioxus::prelude::*;

// #[cfg(feature = "server")]
pub mod server;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct WireguardDevice {
    pub name: String,
    pub group: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub id: u32,
}

#[get("/api/vpn/devices")]
async fn get_vpn_devices() -> dioxus::Result<Vec<WireguardDevice>> {
    let devices = WireguardDevice::load_all()
        .into_iter()
        // .filter(|device| device.group == "vpn")
        .collect();
    Ok(devices)
}
