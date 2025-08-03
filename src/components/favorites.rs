use std::ops::Deref;

use super::super::backend::remove_dog;
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let mut favorites= use_resource(super::super::backend::list_dogs);
// let res = favorites.cloned();
    rsx! {
        div {id: "fivorites",
            div { id: "favorites-container",
                // let res = favorites.value();
                
                for (id, url) in favorites.cloned().unwrap_or(Ok(vec![])).unwrap_or(vec![]){
                    div {
                        key: "{id}",
                        class: "favorite-dog",
                        img {src: "{url}"}
                        button {
                        onclick: move |_| async move {
                            let rez = remove_dog(id).await;
                            // let mut imgs = &mut favorites.write();
                            
                            match rez {
                                Ok(_) => {
                                    
                                    favorites.restart();
                                },
                                Err(_) => {}
                            }
                            // favorites.;
                    // let current_img = img_src.cloned().unwrap_or_default();
                    // img_src.restart();
                    // _ = save_dog(current_img).await
                },
                            "❌"
                        }
                    }
                }
        }
    }
    }
}
