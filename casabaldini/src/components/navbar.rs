use dioxus::prelude::*;
use crate::models::{Menus, MenuItem};
use crate::components::nav_item::NavItem;
use crate::models::get_menu;
use crate::Route;

#[component]
pub fn Navbar() -> Element {
    let menu_res = use_resource(move || get_menu());

    let mut is_open = use_signal(|| false);

    rsx! {
        div { class: "sp-menu",

            div {
                class: "menu-toggle",
                style: "position: absolute; top: -20px;",
                button { r#type: "button", onclick: move |_| is_open.toggle(),
                    span { class: "icon-bar" }
                    span { class: "icon-bar" }
                    span { class: "icon-bar" }
                }
            }

            ul {
                id: "dioxus-menu",
                class: if is_open() { "dioxus-menu show" } else { "dioxus-menu" },

                match &*menu_res.read_unchecked() {
                    Some(Ok(menu_items)) => {

                        rsx! {

                            for item in menu_items {
                                NavItem {
                                    key: "{item.parent.id}",
                                    m: item.parent.clone(),
                                    subitems: item.children.clone(),
                                }
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

        Outlet::<Route> {}
    }
}