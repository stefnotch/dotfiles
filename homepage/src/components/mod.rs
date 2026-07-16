//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.
mod doorbell;
pub use doorbell::Doorbell;

mod wireguard;
pub use wireguard::Wireguard;

mod expand_box;
pub use expand_box::ExpandBox;

mod echo;
pub use echo::Echo;

mod qrcode;
pub use qrcode::QrCode;

mod cat;
pub use cat::Cat;
