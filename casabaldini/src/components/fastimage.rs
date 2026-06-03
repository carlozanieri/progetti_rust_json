use dioxus::prelude::*;

#[component]
pub fn FastImage(name: String, dir: String) -> Element {
    let src = format!("https://json.casabaldini.eu/static/img/{}/{}", dir, name);

    rsx! {
        img {
            src: "{src}",
            style: "width: 100%; height: 100%; display: block; object-fit: cover;",
        }
    }
}