pub mod create_user;
pub mod login;
pub mod character;

use yew::prelude::*;
use yew_router::prelude::*;

use login::Login;
use create_user::CreateUser;
use character::CharacterList;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/createuser")]
    CreateUser,
    #[at("/characters")]
    Characters,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::CreateUser => html! {<CreateUser />},
        AppRoute::Home => html! {<Home />},
        AppRoute::Characters => html! {<CharacterList />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}
#[function_component(Home)]
pub fn home() -> Html {
    html!{
        <>
            {"Home"}
        </>
    }
} 
#[function_component(Nav)]
pub fn nav() -> Html {
    let history = use_history().unwrap();
    let open = false;
    
    let character_list_button = {
        let history = history.clone();
        let onclick = Callback::once
            (move |_| history.push(AppRoute::Characters));
        html! {
            <button {onclick}>{"Character List"}</button>
        }
    };

    html! {
        <>
            {
                if open 
                {
                    character_list_button
                } 
                else 
                {
                    html!{<>{""}</>}
                }
            }
        </>
    }
}
