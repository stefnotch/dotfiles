use crate::components::{Doorbell, Echo, ExpandBox};
use dioxus::prelude::*;

const HOME_ICON: Asset = asset!("/assets/house.svg");
const CAT_PICTURE: Asset = asset!("/assets/cat.svg");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
/// TODO: The house should change if we're not connected with wireguard (aka the backend is unreachable)
#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { display: "flex",
            img {
                src: HOME_ICON,
                alt: "Home Icon",
                style: "border: 0",
                width: 128,
            }
            span { style: "margin-left: 2rem; padding-top: 92px;", "Best Homepage!" }
        }
        div {
            ExpandBox { title: "Doorbell 📸", persistence_key: "doorbell", Doorbell {} }

            ExpandBox { title: "Home Network 🔗", persistence_key: "home_network", HomeNetwork {} }

            ExpandBox { title: "VPN 🔒", persistence_key: "vpn", Vpn {} }

            Echo {}

            Cat {}
        }
    }
}

#[component]
pub fn HomeNetwork() -> Element {
    rsx! {
        div { id: "links",
            a { href: "http://192.168.1.1", "📡 Basement Router" }
            a { href: "http://192.168.1.2", "📡 Upstairs Router" }
            a { href: "http://192.168.8.1", "📡 Radio Router" }

            a { href: "http://192.168.1.16/login.asp", "📸 Doorbell" }

            // TODO: Immich setup guide?
            a { href: "http://127.0.0.1:8080/", "🖼️ Photos" }
        }
    }
}

#[component]
pub fn Vpn() -> Element {
    rsx! {
        div {
            "Download WireGuard!"
        }
    }
}

#[component]
pub fn Cat() -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: end; margin-top: 2rem;",
        img {
            src: CAT_PICTURE,
            alt: "Cat Icon",
            style: "border: 0;",
            width: 128,

        }
    }
    }
}
