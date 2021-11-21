use dominator::{class, html, Dom};
use once_cell::sync::Lazy;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

struct App<'a> {
  headers: Vec<&'a str>,
}

impl App<'_> {
  fn new() -> Arc<Self> {
    Arc::new(Self {
      headers: vec![
        "Jig Pictures",
        "Jig Name",
        "Author",
        "Author's Badge",
        "Date",
        "Instruction Language",
        "Curators",
      ],
    })
  }

  fn render(app: Arc<Self>) -> Dom {
    static MAIN_FLEX_CONTAINER: Lazy<String> = Lazy::new(|| {
      class! {
        .style("display", "flex")
        .style("justify-content", "space-between")
      }
    });

    html!("div", {
      .class(&*MAIN_FLEX_CONTAINER)
      .children(app.headers.iter().map(|header| {
        html!("div", {
          .text(header)
        })
      }))
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
