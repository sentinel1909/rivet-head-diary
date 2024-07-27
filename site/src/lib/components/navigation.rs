// rivet-head-diary site
// src/lib/components/navigation.rs

// dependencies
use crate::views::{Blog, Home, Admin, Root};
use yew::function_component;
use yew::{html, Html};
use yew_router::prelude::*;

// enum to describe the site routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/admin")]
    Admin,
    #[at("/blog")]
    Blog,
}

// function to switch between the site routes
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Root> <Home /> </Root> },
        Route::Admin => html! { <Root> <Admin /> </Root> },
        Route::Blog => html! { <Root> <Blog /> </Root> }, 
    }
}

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
      <>
        <nav>
          <Link<Route> to={Route::Home}>
            { "Home" }
          </Link<Route>>
          <Link<Route> to={Route::Admin}>
            { "Admin" }
          </Link<Route>>
          <Link<Route> to={Route::Blog}>
            { "Blog" }
          </Link<Route>>
        </nav>
      </>
    }
}
