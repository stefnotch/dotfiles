use dioxus::prelude::*;
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use qrcode::render::svg::Color;

#[component]
pub fn QrCode(data: Vec<u8>) -> Element {
    let qr_code = qrcode::QrCode::new(data).unwrap();
    let svg_image = qr_code
        .render::<qrcode::render::svg::Color>()
        .dark_color(Color("#000"))
        .light_color(Color("#ffffff00"))
        .min_dimensions(200, 200)
        .build();

    let encoded = utf8_percent_encode(&svg_image, NON_ALPHANUMERIC).to_string();
    let svg_src = format!("data:image/svg+xml,{encoded}");

    rsx! {
        img {
            src: svg_src,
            alt: "QR Code",
            style: "border: 0",
        }
    }
}
