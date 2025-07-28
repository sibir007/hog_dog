// use bytes::bytes::Bytes;

use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use dioxus::{document::Stylesheet, prelude::*};
use image::ImageReader;
use reqwest::{header, Client};
use web_sys::{console, wasm_bindgen::JsValue};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

const NINJAS_API_RANDOM_IMAGE_KEY: &str = "wmK3q37785hdHuoVLv9BqQ==sqCypyLeK5dROdXz";

// https://api.api-ninjas.com/v1/randomimage
// response = requests.get(api_url, headers={'X-Api-Key': 'YOUR_API_KEY', 'Accept': 'image/jpg'}, stream=True)

// const NINJAS_API_RANDOM_IMAGE_CLIENT: Client = Client::builder().default_headers(headers)
fn main() {
    dioxus::launch(DogApp)
}

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[derive(Clone)]
struct TitleState(String);

#[derive(Clone)]
struct APIClient {
    client: Client,
}

fn create_api_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-Api-Key",
        header::HeaderValue::from_static("wmK3q37785hdHuoVLv9BqQ==sqCypyLeK5dROdXz"),
    );
    headers.insert("Accept", header::HeaderValue::from_static("image/jpg"));
    let client = Client::builder().default_headers(headers).build().unwrap();
    client
}

#[component]
// fn DogApp() -> Element {
fn DogApp() -> Element {
    use_context_provider(|| TitleState("HotDog".to_string()));

    use_context_provider(|| APIClient {
        client: create_api_client(),
    });
    rsx! {
        Stylesheet {href: MAIN_CSS}
        // img {src: HEADER_SVG}
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div {
            id: "title",
            h1 { "{title.0}! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    use_context_provider(|| TitleState("TitleInDegView".to_string()));

    let mut img_src = use_resource(|| async move {
        let req_rez = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await;

        match req_rez {
            Ok(resp) => {
                console::error_1(&JsValue::from_str("get random image response"));
                resp.json::<DogApi>().await.unwrap().message
            
            },
            Err(_) => {
                console::error_1(&JsValue::from_str("get random image error"));
                "".to_string()
            }
        }
    });

    rsx! {
        div {
            id: "dogview",
            img { src: {img_src.cloned().unwrap_or_default() }}
        }
        div {
            id: "buttons",
            button { id: "skip", onclick: move |_| img_src.restart(), "skip" }
            button { id: "save",
                onclick: move |_| async move {
                    let current_img = img_src.cloned().unwrap_or_default();
                    img_src.restart();
                    _ = save_dog(current_img).await
                },

                "save!"
            }
        }
        Title2 {  }

    }
}

#[component]
fn Title2() -> Element {
    let title = use_context::<TitleState>();

    rsx! {
        div {
            id: "title",
            h1 { "{title.0}! 🌭" }
        }
    }
}

pub fn ParentComponent() -> Element {
    let count = use_signal(|| 0);

    rsx! {
        "Count is {count}"
        IncrementButton {
            count
        }
    }
}
#[component]
fn IncrementButton(mut count: Signal<i32>) -> Element {
    rsx! {
        button {
            onclick: move |_| count += 1,
            "Increment"
        }
    }
}

// #[cfg(feature = "server")]
// thread_local! {
//     pub static DB: rusqlite::Connection = {
//         let conn = rusqlite::Connection::open("hotdog.db").ex
//     }
// }

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();
    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}
