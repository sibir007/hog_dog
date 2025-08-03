use dioxus::prelude::*;

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    let mess = segments.join("/");

    rsx! {
        "rout {mess} not found"
    }
}


