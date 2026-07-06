use crate::components::{Doorbell, Echo, ExpandBox};
use dioxus::prelude::*;

const HOME_ICON: Asset = asset!("/assets/house.svg");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
/// TODO: The house should change if we're not connected with wireguard (aka the backend is unreachable)
#[component]
pub fn Home() -> Element {
    rsx! {
        h1 {
            display: "flex",
            img { src: HOME_ICON, alt: "Home Icon", style: "border: 0", width: 128 },
            span {
                style: "margin-left: 2rem; padding-top: 92px;",
                "Best Homepage!"
            }
        }
        div {
            ExpandBox {
                title: "Doorbell 📸",
                persistence_key: "doorbell",
                Doorbell {}
            }

            ExpandBox {
                title: "Home Network 🔗",
                persistence_key: "home_network",
                HomeLinks {}
            }

            Echo {}
        }
    }
}

#[component]
pub fn HomeLinks() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            // Attributes should be defined in the element before any children
            // After all attributes are defined, we can define child elements and components
            div { id: "links",
                // The RSX macro also supports text nodes surrounded by quotes
                a { href: "http://192.168.1.16/login.asp", "📸 Doorbell" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
// $("img_video_main").src = "http://192.168.1.16/cgi-bin/images_cgi?channel=0&user=" + $p("username").value + "&pwd=" + $p("password").value + "&" + Math.random();
