// rivet-head-diary site
// src/lib/components/footer.rs

// dependencies
use yew::function_component;
use yew::{html, Html};

// the footer component
#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
      <footer>
        <section>
          <article>
            <p>{ "Site built with Yew and Axum, hosted on Shuttle" }</p>
            <p>{ "\u{00A9} " } { "Jeffery D. Mitchell" }</p>
          </article>
        </section>
      </footer>
    }
}
