// rivet-head-diary site
// src/lib/views/home.rs

// dependencies
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::function_component;
use yew::{html, use_effect, use_state, Html};

// function which renders home page content
#[function_component(Home)]
pub fn home() -> Html {
    // give the home component state so that it can hold data, initialize with a None variant
    let data = use_state(|| None);

    // utilize the use_effect hook to make a request to the /api/health_check endpoint
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/health_check")
                        .send()
                        .await
                        .expect("Unable to reach the API.");
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    // match on the data and display a message on the home page indicating the API's current status
    match data.as_ref() {
        None => {
            html! {
                <div>{"No response from the API"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <>
                    <section>
                        <h2>{ "Welcome!" }</h2>
                        <article>
                            <p>{"Received API response: "}{data}{"200 OK"}</p>
                        </article>
                    </section>
                </>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from the API: "}{err}</div>
            }
        }
    }
}
