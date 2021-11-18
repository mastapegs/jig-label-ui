use once_cell::sync::Lazy;
use dominator::{Dom, html, class};
use wasm_bindgen::prelude::*;

struct App {}

impl App {
    fn render() -> Dom {
        static MAIN_GRID_CONTAINER: Lazy<String> = Lazy::new(|| class! {
          .style("display", "grid")
        });
      
        html!("div", {
          .class(&*MAIN_GRID_CONTAINER)
          .text("Hello, World!")
        })
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    dominator::append_dom(&dominator::body(), App::render());

    Ok(())
}
