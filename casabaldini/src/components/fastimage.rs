use dioxus::prelude::*;

#[component]
pub fn FastImage(name: String, dir: String) -> Element {
    let src = format!("https://json.casabaldini.eu/static/img/{}/{}", dir, name);

    rsx! {
        if dir == "links" || dir == "ristoranti" {
            img {
                src: "{src}",
                style: "width: 30%; height: 30%; display: block; object-fit: cover;",
            }
        } else {
            img {
                src: "{src}",
                style: "width: 100%; height: 100%; display: block; object-fit: cover;",
            }
        }
    }
}