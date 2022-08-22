use web_sys::HtmlInputElement;
use yew::prelude::*;
use log;
use serde::Serialize;
use yew_hooks::prelude::*;
use reqwest;


#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
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
        let state = state.clone();
        use_async(async move {
            reqwest::Client::new()
                .post("127.0.0.1:5000/user/login")
                .json(&(LoginModel {
                    username: state.username,
                    password: state.password,
                }))
                .send()
                .await
        });
    };
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
        let state = state.clone();
         
        Callback::from(move |_| {
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
