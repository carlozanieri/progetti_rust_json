use crate::prelude::*;
use crate::components::linkutili::Linkutili;


#[component]
pub fn Casabaldini(dir: String) -> Element {
    println!("--- ESECUZIONE COMPONENTE ---");
    pub const BACK_IMG: Asset = asset!("/assets/bgblack.png");
    //let background_image = "{BACK_IMG}";
    let dir = use_signal(|| dir.to_string());
    //let d_resource = dir.clone();
    //let d = d_resource.clone();
    //let sliders = use_resource(move || get_slide(dir.cloned()));
    rsx! {
        body { style: "background-image: url({BACK_IMG}); background-repeat: repeat;",
            div { class: "slider-pro",

                hr {}
                ElencoSliders { dir }
                Linkutili {}
            }
        }
    }
}