// src/lib/renderer/renderer.rs

// dependencies
use once_cell::sync::OnceCell;
use tera::{Tera, Error};

// initializer for the compiled template item, uses a static memory location
static COMPILED_TEMPLATE: OnceCell<Tera> = OnceCell::new();

// compile the tera template, saving it in static memory
pub fn compile_diary_template() -> Result<&'static Tera, Error> {
    COMPILED_TEMPLATE.get_or_try_init(|| {
        let mut diary_template = Tera::new("templates/**/*")?;
        diary_template.add_template_file("templates/index.html", None)?;
        Ok(diary_template)
    })
}