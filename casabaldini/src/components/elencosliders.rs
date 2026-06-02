use crate::prelude::*;
use crate::components::fastimage::FastImage;
use crate::document::eval;
use crate::models::get_slide;
#[component]
pub fn ElencoSliders(dir: String) -> Element {
    let dir = use_signal(|| dir.to_string());
    let sliders_res = use_resource(move || get_slide(dir.cloned()));

    let inizializza_slider = move |_| {
        spawn(async move {
            let _ = eval(r#"
                var $slider = $('#example1');
                if ($slider.length > 0 && typeof $.fn.sliderPro !== 'undefined') {
                    $slider.sliderPro({
                        width: 960,
                        height: 500,
                        arrows: true,
                        buttons: true,
                        autoplay: true,
                        autoHeight: false,
                        forceSize: 'none', // Fondamentale per non andare a tutto schermo
                        imageScaleMode: 'cover',
                        centerImage: true
                    });
                }
            "#);
        });
    };

    rsx! {
        match &*sliders_res.read_unchecked() {
            Some(Ok(list)) => rsx! {

                div { style: "width: 100%; max-width: 920px; margin: 50px auto; position: relative; clear: both;",

                    div { id: "example1", class: "slider-pro", onmounted: inizializza_slider,

                        div { class: "sp-slides",
                            for s in list {
                                div { class: "sp-slide", key: "{s.id.clone()}",
                                    FastImage { name: s.img.clone(), dir: {dir.clone()} }

                                    h3 {
                                        class: "sp-layer sp-black sp-padding",
                                        "data-horizontal": "40",
                                        "data-vertical": "10%",
                                        "data-show-transition": "left",
                                        "data-hide-transition": "left",
                                        "{s.titolo}"

                                    }

                                    p {
                                        class: "sp-layer sp-white sp-padding hide-medium-screen",
                                        "data-horizontal": "40",
                                        "data-vertical": "22%",
                                        "data-show-transition": "left",
                                        "data-hide-transition": "left",
                                        "{s.caption}"
                                    }

                                    p {
                                        style: "background-color:#330101;color:#ffffff;",
                                        class: "sp-layer sp-white sp-padding hide-small-screen",
                                        "data-horizontal": "40",
                                        "data-vertical": "34%",
                                        "data-show-transition": "left",
                                        "{s.testo}"
                                        "--"
                                        "{s.img.clone()}"
                                        "--"
                                        "{dir.clone()}"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => rsx! {
                img { src: CLESSIDRA, id: "header" }
            },
        } // Chiusura match
    } // Chiusura rsx!
} 