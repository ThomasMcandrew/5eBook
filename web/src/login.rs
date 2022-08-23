use web_sys::HtmlInputElement;
use yew::prelude::*;
use log;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew_hooks::prelude::*;
use reqwest;

use crate::api_caller;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LoginModel {
    username: String,
    password: String,
}


//https://github.com/jetli/rust-yew-realworld-example-app/blob/master/crates/conduit-wasm/src/routes/login.rs
//good example
#[function_component(Login)]
pub fn login() -> Html {
    let state = use_state(|| LoginModel {
        username: "".to_string(),
        password: "".to_string(),
    });
    let login_task = {
        log::info!("in login task level 1");
        let state = state.clone();
        use_async(async move {
            log::info!("level 2");
            api_caller::post::<LoginModel,LoginModel>
                (String::from("http://127.0.0.1:5000/user/login"),
                LoginModel{
                    username: state.username.clone(),
                    password: state.password.clone(),
                }).await
        })
    };
    use_effect_with_deps(
        move |state| {
            ||()
        },
        state.clone(),
    );

    let onchange_username = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(LoginModel { 
                username: input.value(),
                password: state.password.clone(),
            });
        })
    };
    let onchange_password = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            state.set(LoginModel { 
                username: state.username.clone(),
                password: input.value(),
            });
        })
    };
    let onsubmit = {
        Callback::from(move |_| {
            log::info!("in here");
            login_task.run();
        })
    };
    html! {
        <>
            <div>
                {"username: "}
                <input onchange={onchange_username} 
                value={(*state).username.clone()}  />
            </div>
            <div>
                {"password: "}
                <input onchange={onchange_password} 
                value={(*state).password.clone()} type="password"  />
            </div>
            <div>
                <button onclick={onsubmit}>{"Login"}</button>
            </div>
        </>
    }
}
