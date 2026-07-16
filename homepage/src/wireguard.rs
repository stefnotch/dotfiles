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

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct WireguardDeviceWithPrivateKey {
    pub device: WireguardDevice,
    pub private_key: String,
}
