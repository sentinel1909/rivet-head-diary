// rivet-head-diary site
// src/lib/components/navigation.rs

// dependencies
use crate::views::{Blog, Diary, Home, Login, Root, Unauthorized};
use yew::function_component;
use yew::{html, Html};
use yew_router::prelude::*;

// enum to describe the site routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/blog")]
    Blog,
    #[at("/diary")]
    Diary,
    #[at("/unauthorized")]
    Unauthorized,
}

// function to switch between the site routes
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Root> <Home /> </Root> },
        Route::Login => html! { <Root> <Login /> </Root> },
        Route::Blog => html! { <Root> <Blog /> </Root> },
        Route::Diary => html! { <Root> <Diary /> </Root> }, 
        Route::Unauthorized => html! { <Root> <Unauthorized /> </Root>}
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
          <Link<Route> to={Route::Login}>
            { "Login" }
          </Link<Route>>
          <Link<Route> to={Route::Blog}>
            { "Blog" }
          </Link<Route>>
        </nav>
      </>
    }
}
