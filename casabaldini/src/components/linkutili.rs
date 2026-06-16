use crate::prelude::*;
use crate::components::fastimage::FastImage;
use dioxus::prelude::asset;
use crate::models::get_links;
#[component]
pub fn Linkutili() -> Element {
    let link_res = use_resource(move || get_links());
    let dir ="links";
rsx! {
    match &*link_res.read_unchecked() {
        Some(Ok(lista)) => rsx! {
            div { class: "marquee",
                div { class: "marquee-content",
                    for l in lista {
                        // Se vuoi usare variabili qui dentro, devi aprire un blocco { }
                        // Ma chiamare il componente direttamente è più pulito:
                        a {
                            key: "{l.id}",
                            href: "{l.link}",
                            style: "color:#ffffff; font-size: 1.5em; font-weight: bold; display: inline-flex; align-items: center; margin-right: 50px;",

                            span { style: "color:#ffffff; font-size: 1.5em; font-weight: bold;  align-items: center; margin-right: 50px;",

                                FastImage { name: l.img.clone(), dir }
                                span { style: "", "{l.titolo}" }

                            }

                        }
                    }
                }
            }
        },
        _ => rsx! {
            img { src: CLESSIDRA, id: "header" }
        },
    }

}	

}

