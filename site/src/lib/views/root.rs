// rivet-head-diary site
// src/lib/views/home.rs

// dependencies
use crate::components::{Footer, Header, Nav};
use yew::function_component;
use yew::{html, Children, Html, Properties};

// struct type to represent the properties for the Root component,
// enables the passing of child components
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

// the root component
#[function_component(Root)]
pub fn root(props: &Props) -> Html {
    html! {
      <div class="container mx-auto">
        <Header />
        <Nav />
        <main>
          {props.children.clone()}
        </main>
        <Footer />
      </div>
    }
}
