// rivet-head-diary site
// src/main.rs

// dependencies
use rivet_head_diary_site_lib::app::App;

// main function
fn main() {
    // set up logging, send log level, source line, and filename to the web browser console
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    // allows debugging Rust panic messages in a web browser
    console_error_panic_hook::set_once();

    // render the app component
    yew::Renderer::<App>::new().render();
}
