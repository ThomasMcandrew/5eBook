use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_hooks::prelude::*;

use crate::api_caller;
use crate::models::login::LoginModel;
use crate::routes::AppRoute;

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
            api_caller::post::<LoginModel>
                (String::from("user/login"),
                LoginModel{
                    username: state.username.clone(),
                    password: state.password.clone(),
                }).await
        })
    };
    use_effect_with_deps(
        move |_| {
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
            <div>
            <Link<AppRoute> 
                to={AppRoute::CreateUser}
                >{ "sign up" }</Link<AppRoute>>
            </div>
        </>
    }
}
