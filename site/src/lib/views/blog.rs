// rivet-head-diary site
// src/lib/view/blog.rs

// dependencies
use yew::function_component;
use yew::{html, Html};

// function which renders the login page view
#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
      <>
        <section>
          <p>{ "The blog content goes here..." }</p>
        </section>
      </>
    }
}
