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
        static FLEX_ITEM: Lazy<String> = Lazy::new(|| {
            class! {
              .style("flex", "1 1 0")
              .style("border", "solid 1px #eaebef")
              .style("padding", "5px")
            }
        });
        static HEADER_FLEX_CONTAINER: Lazy<String> = Lazy::new(|| {
            class! {
              .style("display", "flex")
              .style("justify-content", "space-between")
              .style("font-weight", "700")
              .style("letter-spacing", "-0.16px")
              .style("color", "#2565d5")
            }
        });

        static JIGS_CONTAINER: Lazy<String> = Lazy::new(|| {
            class! {
              .style("display", "flex")
              .style("justify-content", "space-between")
            }
        });

        static MAIN_CONTAINER: Lazy<String> = Lazy::new(|| {
            class! {
              .style("background", "#f3f8fe")
              .style("font-family", "sans-serif")
              .style("border", "solid 1px #c4d9f7")
            }
        });

        html!("div", {
          .class(&*MAIN_CONTAINER)
          .children([
            html!("div", {
              .class(&*HEADER_FLEX_CONTAINER)
              .children(app.headers.iter().map(|header| {
                html!("div", {
                  .class(&*FLEX_ITEM)
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
                      .class(&*FLEX_ITEM)
                      .text(&jig.jig_name)
                    }),
                    html!("div", {
                      .class(&*FLEX_ITEM)
                      .text(&jig.author)
                    }),
                    html!("div", {
                      .class(&*FLEX_ITEM)
                      .text(&jig.author_badge)
                    }),
                    html!("div", {
                      .class(&*FLEX_ITEM)
                      .text(&jig.date)
                    }),
                    html!("div", {
                      .class(&*FLEX_ITEM)
                      .text(&jig.language)
                    }),
                    html!("div", {
                      .class(&*FLEX_ITEM)
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

    let jig_headers = vec![
        String::from("Jig Name"),
        String::from("Author"),
        String::from("Author's Badge"),
        String::from("Date"),
        String::from("Instruction Language"),
        String::from("Curators"),
    ];

    let jigs: Vec<JigData> = vec![
        JigData {
            jig_name: String::from("Hebrew Letters"),
            author: String::from("Michael Wikes"),
            author_badge: String::from("JI Team"),
            date: String::from("Aug. 5, 2020"),
            language: String::from("English (American)"),
            curators: vec![String::from("Anat (13.7.21)")],
        },
        JigData {
            jig_name: String::from("Hebrew Numbers & Balloons"),
            author: String::from("Michael Wikes"),
            author_badge: String::from("JI Team"),
            date: String::from("Aug. 5, 2020"),
            language: String::from("English (American)"),
            curators: vec![String::from("Anat (13.7.21)")],
        },
        JigData {
            jig_name: String::from("Hebrew Letters"),
            author: String::from("Michael Wikes"),
            author_badge: String::from("JI Team"),
            date: String::from("Aug. 5, 2020"),
            language: String::from("English (American)"),
            curators: vec![String::from("Anat (13.7.21)")],
        },
        JigData {
            jig_name: String::from("Hebrew Letters"),
            author: String::from("Michael Wikes"),
            author_badge: String::from("JI Team"),
            date: String::from("Aug. 5, 2020"),
            language: String::from("English (American)"),
            curators: vec![String::from("Anat (13.7.21)")],
        },
        JigData {
            jig_name: String::from("Hebrew Letters"),
            author: String::from("Michael Wikes"),
            author_badge: String::from("JI Team"),
            date: String::from("Aug. 5, 2020"),
            language: String::from("English (American)"),
            curators: vec![String::from("Anat (13.7.21)")],
        },
        JigData {
            jig_name: String::from("Hebrew Letters"),
            author: String::from("Michael Wikes"),
            author_badge: String::from("JI Team"),
            date: String::from("Aug. 5, 2020"),
            language: String::from("English (American)"),
            curators: vec![String::from("Anat (13.7.21)")],
        },
    ];
    let app = App::new(jig_headers, jigs);
    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
