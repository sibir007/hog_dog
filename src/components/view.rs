use dioxus::prelude::*;
use reqwest;
// use web_sys::{console, wasm_bindgen::JsValue};
use crate::backend::save_dog;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}


#[component]
pub fn DogView() -> Element {
    // use_context_provider(|| TitleState("TitleInDegView".to_string()));

    let mut img_src = use_resource(|| async move {
        let req_rez = reqwest::get("https://dog.ceo/api/breeds/image/random").await;

        match req_rez {
            Ok(resp) => {
                let mess = resp.json::<DogApi>().await.unwrap().message;
                // console::log_1(&JsValue::from_str(format!("response {}", mess).as_str()));
                mess
            }
            Err(_) => {
                // console::error_1(&JsValue::from_str("get random image error"));
                "".to_string()
            }
        }
    });

    rsx! {
        div {
            id: "dogview",
            img { src: "{img_src.cloned().unwrap_or_default()}"}
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
        // Title2 {  }

    }
}
