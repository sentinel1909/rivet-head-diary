// src/lib/view/login.rs

// dependencies
use crate::components::input::InputField;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json;
use web_sys::{HtmlInputElement, window};
use yew::{Callback, function_component, Html, html, Properties, SubmitEvent, use_node_ref, use_state};

// a struct type to represent the login form
#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct LoginForm {
    pub client_id: String,
    pub client_secret: String,
}

// a struct type to represent the login response
#[derive(Debug, Deserialize)]
struct LoginResponse {
    access_token: String,
}

#[function_component(Admin)]
pub fn admin() -> Html {
    let _login_form = use_state(|| {LoginForm::default()});

    let client_id_ref = use_node_ref();
    let client_secret_ref = use_node_ref();

    let onsubmit = {
        let client_id_ref = client_id_ref.clone();
        let client_secret_ref = client_secret_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let client_id = client_id_ref.cast::<HtmlInputElement>().unwrap().value();
            let client_secret = client_secret_ref.cast::<HtmlInputElement>().unwrap().value();

            let login_form = LoginForm {
                client_id,
                client_secret,
            };

            wasm_bindgen_futures::spawn_local(async move {
                todo!();
            });
        })
    };
    
    html! {
        <>
            <form {onsubmit}>
                <InputField input_node_ref={client_id_ref} label={"Client ID".to_owned()} name={"client_id"} field_type={"text"} />
                <InputField input_node_ref={client_secret_ref} label={"Client Secret".to_owned()} name={"client_secret"} field_type={"password"} />
                <button type="submit">{"Login"}</button>
            </form>
        </>
    }
}





