// rivet-head-diary site
// src/lib/views/diary.rs

// dependencies
use crate::components::navigation::Route;
use yew::function_component;
use yew::{html, Html};
use yew_router::prelude::*;

// function which renders the login page view
#[function_component(Diary)]
pub fn diary() -> Html {
    html! {
      <>
        <section>
          <p>{ "The diary entry admin page." }</p>
        </section>
        <br />
        <section>
          <p><Link<Route> to={ Route::Home }>{ "Logout" }</Link<Route>></p>
        </section>
      </>
    }
}