use web_sys::HtmlInputElement;
use yew::prelude::*;
use log;
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
struct LoginModel {
    username: String,
    password: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let state = use_state(|| LoginModel {
        username: "".to_string(),
        password: "".to_string(),
    });
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
            let foo = reqwest::Client::new()
                .post("http://127.0.0.1:5000/user/login")
                .json::<LoginModel>(&state)
                .send();
        //    let foo = reqwest::Client::new()
        //        .post("http::127.0.0.1:5000")
        //        .json(&state)
        //        .send()
        //        .await?
        //        .json()
        //        .await?;

            log::info!("{:?}",state);
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
