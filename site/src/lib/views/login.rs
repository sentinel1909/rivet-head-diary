// src/lib/view/login.rs

// dependencies
use crate::components::input::InputField;
use gloo_net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{Callback, function_component, Html, html, Properties, SubmitEvent, use_node_ref, use_state};

// a struct type to represent the login form
#[derive(Clone, PartialEq, Properties, Debug, Default, Serialize, Deserialize)]
pub struct LoginForm {
    pub client_id: String,
    pub client_secret: String,
}

// a struct type to represent the login response
#[derive(Deserialize)]
struct LoginResponse {
    access_token: String,
}

#[function_component(Login)]
pub fn login() -> Html {
    let login_form = use_state(|| {LoginForm::default()});

    let client_id_ref = use_node_ref();
    let client_secret_ref = use_node_ref();

    let onsubmit = {
        let _login_form = login_form.clone();
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

            log::info!("Login form {:?}", &login_form.clone());

            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("/api/login")
                    .headers({
                        let headers = Headers::new();
                        headers.set("Content-Type", "application/json");
                        headers
                    })
                    .body(JsValue::from(
                        serde_json::to_string(&login_form).unwrap(),
                    ))
                    .unwrap()
                    .send()
                    .await
                    .unwrap();

                if response.status() == 200 {
                    let login_response: LoginResponse = response.json().await.unwrap();
                    web_sys::window()
                        .unwrap()
                        .local_storage()
                        .unwrap()
                        .expect("Unable to access local storage.")
                        .set_item("access_token", &login_response.access_token)
                        .unwrap();
                    web_sys::window()
                        .unwrap()
                        .location()
                        .set_href("/api/protected")
                        .unwrap();
                } else {
                    log::info!("Unathorized");
                }
            })

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





