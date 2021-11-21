use dominator::{class, html, Dom};
use once_cell::sync::Lazy;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

struct JigData {
  jig_name: String,
  author: String,
  author_badge: String,
  date: String,
  language: String,
  curators: Vec<String>,
}

struct App {
  headers: Vec<String>,
  jigs: Vec<JigData>,
}

impl App {
  fn new(headers: Vec<String>, jigs: Vec<JigData>) -> Arc<Self> {
    Arc::new(Self {
      headers: headers,
      jigs: jigs,
    })
  }

  fn render(app: Arc<Self>) -> Dom {
    static HEADER_FLEX_CONTAINER: Lazy<String> = Lazy::new(|| {
      class! {
        .style("display", "flex")
        .style("justify-content", "space-between")
      }
    });

    static JIGS_CONTAINER: Lazy<String> = Lazy::new(|| {
      class! {
        .style("display", "flex")
        .style("justify-content", "space-between")
      }
    });

    html!("div", {
      .children([
        html!("div", {
          .class(&*HEADER_FLEX_CONTAINER)
          .children(app.headers.iter().map(|header| {
            html!("div", {
              .text(header)
            })
          }))
        }),
        html!("div", {
          .children(app.jigs.iter().map(|jig| {
            html!("div", {
              .class(&*JIGS_CONTAINER)
              .children([
                html!("div", {
                  .text(&jig.jig_name)
                }),
                html!("div", {
                  .text(&jig.author)
                }),
                html!("div", {
                  .text(&jig.author_badge)
                }),
                html!("div", {
                  .text(&jig.date)
                }),
                html!("div", {
                  .text(&jig.language)
                }),
                html!("div", {
                  .text(&jig.curators[0])
                }),
              ])
            })
          }))
        }),
      ])
    })
  }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  let jigs: Vec<JigData> = vec![JigData {
    jig_name: String::from("name"),
    author: String::from("author"),
    author_badge: String::from("badge"),
    date: String::from("date"),
    language: String::from("language"),
    curators: vec![String::from("person1")],
  }];
  let jig_headers = vec![
    String::from("Jig Name"),
    String::from("Author"),
    String::from("Author's Badge"),
    String::from("Date"),
    String::from("Instruction Language"),
    String::from("Curators"),
  ];
  let app = App::new(jig_headers, jigs);
  dominator::append_dom(&dominator::body(), App::render(app));

  Ok(())
}
