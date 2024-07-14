// rivet-head-diary site
// src/lib/app.rs

// dependencies
use crate::components::navigation::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

// function to render the app
#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <Switch<Route> render={switch} />
      </BrowserRouter>
    }
}
