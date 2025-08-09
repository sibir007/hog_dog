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





fn DogGrid() -> Element {
    rsx! {

        SuspenseBoundary {

            fallback: |_| rsx! {
                div {
                    width: "100%",
                    height: "100%",
                    display: "flex",
                    align_items: "center",
                    justify_content: "center",
                    "Loading..."
                }
            },

            div {
                display: "flex",
                flex_direction: "column",
                // Here you would typically map over a list of dog images
                // For simplicity, we are just showing the DogView component
                // You can replace this with actual dog images from your data source
                DogViewGrid {}
            }


         }
    }
}

#[derive(serde::Deserialize)]
enum BreedResponse {
    
}  (String);



#[component]
pub fn DogViewGrid() -> Element {
    let response = use_resource(move || async move {
        // Artificially slow down the request to make the loading indicator easier to seer
        // gloo_timers::future::TimeoutFuture::new(1000).await;
        reqwest::Client::new()
            .get(format!("https://dog.ceo/api/breed/{breed}/images"))
            .send()
            .await?
            .json::<BreedResponse>()
            .await
    })
    // Calling .suspend()? will suspend the component and return early while the future is running
    .suspend()?;

    // Then you can just handle the happy path with the resolved future
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            match &*response.read() {
                Ok(urls) => rsx! {
                    for image in urls.iter().take(3) {
                        img {
                            src: "{image}",
                            width: "100px",
                            height: "100px",
                        }
                    }
                },
                Err(err) => rsx! { "Failed to fetch response: {err}" },
            }
        }
    }
}
