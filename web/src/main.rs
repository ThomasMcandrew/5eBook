use yew::prelude::*;
use wasm_logger;


mod login;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Counter {
    count: u32,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <login::Login/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
