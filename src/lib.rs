use dominator::{class, html, Dom};
use futures_signals::signal::Mutable;
use once_cell::sync::Lazy;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

struct App<'a> {
    headers: Mutable<Vec<&'a str>>,
}

impl App<'_> {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            headers: Mutable::new(vec![
                "Jig Pictures",
                "Jig Name",
                "Author",
                "Author's Badge",
                "Date",
                "Instruction Language",
                "Curators",
            ]),
        })
    }

    fn render(state: Arc<Self>) -> Dom {
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

    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
