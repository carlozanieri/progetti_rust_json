use dioxus::prelude::*;
use crate::models::{Menus, MenuItem};
use crate::components::nav_item::NavItem;
//use crate::components::topo::MenuHover;
use crate::models::get_menu;

pub fn MenuHover() -> Element {
    // Stato per tracciare se il menu è aperto o chiuso
    let mut is_open = use_signal(|| false);
    let menu_res = use_resource(move || get_menu());

    rsx! {
        div {
            // Contenitore principale
            position: "relative",

            // Eventi per aprire e chiudere il menu
            onmouseover: move |_| is_open.set(true),
            onmouseleave: move |_| is_open.set(false),

            img {
                src: "https://img.icons8.com/?size=32&id=QTMCombNgorJ&format=png",
                alt: "MENU",
            }

            // Condizione per mostrare il menu
            if is_open() {
                div {
                    position: "absolute",
                    top: "100%",
                    left: "0",
                    min_width: "800%",
                    background_color: "#290300",
                    box_shadow: "0px 8px 16px 0px rgba(0,0,0,0.2)",
                    padding: "12px",
                    z_index: "1",

                    ul {
                        match &*menu_res.read_unchecked() {
                            Some(Ok(menu_items)) => {

                                rsx! {

                                    for item in menu_items {

                                        NavItem {

                                            key: "{item.parent.id}",
                                            m: item.parent.clone(),

                                            subitems: item.children.clone(),
                                        }
                                        "\u{00A0}"
                                    }

                                }
                            }
                            Some(Err(e)) => rsx! {
                                li { "Errore: {e}" }
                            },
                            None => rsx! {
                                li { "Caricamento..." }
                            },
                        }
                    }
                }
            }
        }
    }
}

pub fn SubMenuHover() -> Element {
    let mut is_open = use_signal(|| false);
    let menu_res = use_resource(move || get_menu());

    rsx! {
        div {
            position: "relative",

            onmouseenter: move |_| is_open.set(true),
            onmouseleave: move |_| is_open.set(false),

            img {
                src: "https://img.icons8.com/?size=32&id=QTMCombNgorJ&format=png",
                alt: "MENU",
            }

            if is_open() {
                div {
                    position: "absolute",
                    top: "100%",
                    left: "0",
                    min_width: "800%",
                    background_color: "#290300",
                    box_shadow: "0px 8px 16px 0px rgba(0,0,0,0.2)",
                    padding: "12px",
                    z_index: "1",

                    ul {
                        match &*menu_res.read_unchecked() {
                            Some(Ok(menu_items)) => {
                                rsx! {
                                    for item in menu_items {
                                        NavItem {
                                            key: "{item.parent.id}",
                                            m: item.parent.clone(),
                                            subitems: item.children.clone(),
                                        }
                                        "\u{00A0}"
                                    }
                                }
                            }
                            Some(Err(e)) => rsx! {
                                li { "Errore: {e}" }
                            },
                            None => rsx! {
                                li { "Caricamento..." }
                            },
                        }
                    }
                }
            }
        }
    }
}
