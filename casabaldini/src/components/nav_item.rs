use dioxus::prelude::*;
use crate::models::Menus;

#[derive(Props, Clone, PartialEq)]
pub struct NavItemProps {
    pub m: Menus,
    pub subitems: Vec<Menus>,
}

#[component]
pub fn NavItem(props: NavItemProps) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        li {
            class: "nav-item",

            onmouseenter: move |_| is_open.set(true),
            onmouseleave: move |_| is_open.set(false),

            a {
                href: "{props.m.link}",
                onclick: move |evt| {
                    if !props.subitems.is_empty() {
                        evt.prevent_default();
                        is_open.toggle();
                    }
                },
                "{props.m.titolo}"
            }

            if is_open() && !props.subitems.is_empty() {
                ul { class: "submenu",
                    for s in props.subitems.iter() {
                        li {
                            a { href: "{s.link}", "{s.titolo}" }
                        }
                    }
                }
            }
        }
    }
}