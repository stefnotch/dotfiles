#[cfg(feature = "server")]
pub mod doorbell;
pub mod wireguard;

pub const DOORBELL_IP: &str = "192.168.1.16";
pub const DOORBELL_USERNAME: &str = "admin";
