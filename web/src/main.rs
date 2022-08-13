use yew::prelude::*;
use yew::{html, Callback};

#[function_component(Foo)]
fn foo() -> Html {
    let mut i = 0;
    html! {
        <>
            <h1>{i}</h1>
            <button onclick={Callback::from(|_| ())}>
                {"Hello"}
            </button>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <Foo/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
