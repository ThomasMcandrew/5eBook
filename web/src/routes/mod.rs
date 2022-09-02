pub mod create_user;
pub mod login;

use yew::prelude::*;
use yew_router::prelude::*;

use login::Login;
use create_user::CreateUser;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/createuser")]
    CreateUser,
    //#[at("/")]
    //Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::CreateUser => html! {<CreateUser />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}
