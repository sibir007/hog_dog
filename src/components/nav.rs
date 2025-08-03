use super::favorites::Favorites;
use super::not_found::PageNotFound;
use super::view::DogView;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    
    #[layout(NavBar)]

        #[route("/")]
        DogView,

        #[route("/favorites")]
        Favorites,

        #[route("/:..segments")]
        PageNotFound { segments: Vec<String> },
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {id: "title",
                
            Link { to: Route::DogView,
                h1 {"🌭 HotDog! "}
            }
            Link { id: "heart", 
                to: Route::Favorites,
                h1 {"♥️"}
            }
        }
        Outlet::<Route> {}
    }
}
