// #[component]
// fn DogView() -> Element {
//     use_context_provider(|| TitleState("TitleInDegView".to_string()));

//     let mut img_src = use_resource(|| async move {
//         let client = use_context::<APIClient>().client;
//         let response = client
//             .get("https://api.api-ninjas.com/v1/randomimage")
//             .send()
//             .await
//             .unwrap();

//         console::log_1(&format!("requvers don {}", response.status()).into());

//         let image_bytes = response.bytes().await.unwrap();

//         let img = ImageReader::new(Cursor::new(image_bytes))
//             .with_guessed_format()
//             .unwrap()
//             .decode()
//             .unwrap();

//         let mut image_data: Vec<u8> = Vec::new();
//         img.write_to(&mut Cursor::new(&mut image_data), image::ImageFormat::Png)
//             .unwrap();
//         let res_base64 = general_purpose::STANDARD.encode(image_data);
//         // format!("data:image/png;base64,{}", res_base64)
//         res_base64
//     });

//     rsx! {
//         div {
//             id: "dogview",
//             img { src: format!("data:image/png;base64,{}",  img_src.cloned().unwrap_or_default()) }
//         }
//         div {
//             id: "buttons",
//             button { id: "skip", onclick: move |_| img_src.restart(), "skip" }
//             button { id: "save",
//                 onclick: move |_| async move {
//                     let current_img = img_src.cloned().unwrap();
//                     img_src.restart();
//                     _ = save_dog(current_img).await
//                 },

//                 "save!"
//             }
//         }
//         Title2 {  }

//     }
// }