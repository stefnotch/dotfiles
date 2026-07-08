use dioxus::prelude::*;

use crate::wireguard::WireguardDevice;

// #[get("/api/vpn/devices")]
async fn get_vpn_devices() -> dioxus::Result<Vec<WireguardDevice>> {
    let devices = WireguardDevice::load_all()
        .into_iter()
        // .filter(|device| device.group == "vpn")
        .collect();
    Ok(devices)
}
