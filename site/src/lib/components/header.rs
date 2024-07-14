// crusty-rustacean site
// src/lib/components/header.rs

// dependencies
use yew::function_component;
use yew::{html, Html};

// the home page

#[function_component(Header)]
pub fn header() -> Html {
    html! {
      <header>
        <h1 class="text-xl">{ "Rivet Head" }</h1>
        <h2>{ "The music diary...and sort of blog..." }</h2>
      </header>
    }
}
