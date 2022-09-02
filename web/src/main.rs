use yew::prelude::*;
use yew_router::prelude::*;
use wasm_logger;
use tokio;

mod api;
mod routes;
mod models;

use crate::routes::{switch,AppRoute};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Counter {
    count: u32,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <BrowserRouter>
                <Switch<AppRoute> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}
#[tokio::main]
async fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
