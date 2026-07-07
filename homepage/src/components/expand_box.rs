use dioxus::prelude::*;

use crate::storage::use_persistent;

#[component]
pub fn ExpandBox(title: String, persistence_key: String, children: Element) -> Element {
    let mut expanded = use_persistent(format!("{}_expanded", persistence_key), || false);
    rsx! {
        label { class: "checkbox-summary", style: "font-size: xx-large;",
            input {
                style: "font-size: 1rem;",
                r#type: "checkbox",
                checked: "{expanded.get()}",
                onchange: move |event| {
                    expanded.set(event.checked());
                },
            }
            "{title}"
        }
        if expanded.get() {
            blockquote { style: "margin: 0 0 1rem 1rem;", {children} }
        }
    }
}
