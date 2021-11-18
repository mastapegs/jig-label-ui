use dominator::{class, html, Dom};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

struct App {}

impl App {
    fn render() -> Dom {
        static MAIN_GRID_CONTAINER: Lazy<String> = Lazy::new(|| {
            class! {
              .style("display", "grid")
              .style("grid-template-columns", "[jig-pics] 200px [jig-name] auto [author] auto [author-badge] auto [date] auto [instr-lang] auto [curators] auto")
            }
        });

        html!("div", {
          .class(&*MAIN_GRID_CONTAINER)
          .children(&mut [
            html!("div", {
              .text("Jig Pictures")
            }),
            html!("div", {
              .text("Jig Name")
            }),
            html!("div", {
              .text("Author")
            }),
            html!("div", {
              .text("Author's Badge")
            }),
            html!("div", {
              .text("Date")
            }),
            html!("div", {
              .text("Instruction Language")
            }),
            html!("div", {
              .text("Curators")
            }),
          ])
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
