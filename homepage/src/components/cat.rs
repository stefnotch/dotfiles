use dioxus::prelude::*;

const CAT_PICTURE: Asset = asset!("/assets/cat.svg");

#[component]
pub fn Cat() -> Element {
    rsx! {
        div { style: "display: flex; justify-content: end; margin-top: 2rem;",
            img {
                src: CAT_PICTURE,
                alt: "Cat Icon",
                style: "border: 0;",
                width: 128,
                class: "animated-cat",
                tabindex: "0",
            }
        }
    }
}
