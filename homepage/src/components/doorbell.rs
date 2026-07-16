use crate::server::{DOORBELL_IP, DOORBELL_USERNAME};
use std::time::Duration;

use dioxus::prelude::*;

use crate::{storage::use_persistent, timers::use_interval};

#[component]
pub fn Doorbell() -> Element {
    let mut doorbell_password = use_persistent("doorbell-password", || String::new());

    let mut image_src = use_signal(|| String::new());
    let mut counter = use_signal(|| 0);
    let _doorbell_timer = use_interval(Duration::from_millis(1000), move |()| {
        let id = counter();
        counter.set(id + 1);
        image_src.set(format!(
            "/api/doorbell/picture?password={}&id={}",
            doorbell_password.get(),
            id,
        ));
    });

    rsx! {
        // create a video feed from the doorbell
        "Connect to the doorbell!"
        br {}
        input {
            r#type: "password",
            placeholder: "Password",
            minlength: "8",
            size: "20",
            value: "{doorbell_password.get()}",
            oninput: move |event| {
                doorbell_password.set(event.value());
            },
        }
        br {}
        if doorbell_password.get().len() >= 8 {
            img {
                alt: "Doorbell Video Feed",
                style: "max-width: min(640px, 100%); height: auto;",
                src: "{image_src.read()}",
            }
            br {}
            a {
                href: "http://{DOORBELL_USERNAME}:{doorbell_password.get()}@{DOORBELL_IP}/login.asp",
                target: "_blank",
                "Settings ⚙️"
            }
        }
    }
}
