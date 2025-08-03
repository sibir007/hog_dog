use dioxus::{document::Stylesheet, prelude::*};
// use reqwest::{header, Client};
// use components::view::{DogView, PageNotFound};
use components::nav::Route;

mod backend;
mod components;


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");


// #[component]
// fn PageNotFound(segments: Vec<String>) -> Element {
//     rsx!()
// }
#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }

        // 📣 delete Title and DogView and replace it with the Router component.
        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(app)
}



// #[component]
// // fn DogApp() -> Element {
// fn DogView() -> Element {
//     use_context_provider(|| TitleState("HotDog".to_string()));

//     use_context_provider(|| APIClient {
//         client: create_api_client(),
//     });
//     rsx! {
//         Stylesheet {href: MAIN_CSS}
//         // img {src: HEADER_SVG}
//         Title {}
//         DogView {}
//     }
// }


