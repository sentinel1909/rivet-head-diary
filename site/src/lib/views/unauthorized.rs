// rivet-head-diary site
// src/lib/views/unauthorized.rs

// dependencies
use yew::function_component;
use yew::{html, Html};

// function which renders the login page view
#[function_component(Unauthorized)]
pub fn unauthorized() -> Html {
    html! {
      <>
        <section>
          <p>{ "Unauthorized to access the admin area." }</p>
        </section>
      </>
    }
}