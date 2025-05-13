use dioxus::prelude::*;
use group::rt::{Finite, Group, GroupElement, ZnMul, ZnMulElement};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut n = use_signal(|| 2usize);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        input {
          type: "number",
          value: n,
          oninput: move |e| {
            if let Ok(num) = e.value().parse::<usize>() {
              n.set(num);
            }
          }
        }

        GroupView { n }

    }
}

#[component]
pub fn GroupView(n: ReadOnlySignal<usize>) -> Element {
    let g = ZnMul::new(n());
    rsx! {
        div {
            h1 {
              "Z mod {n} Z"
            }
            table {
              tr {
                th { "Element" }
                th { "Order" }
                th { "Subgroup" }
              }
              for i in 1..n() {
                if g.element(i).is_some() {
                  GroupElementView { i, g }
                }
              }
            }
        }
    }
}

#[component]
pub fn GroupElementView(i: usize, g: ZnMul) -> Element {
    let element = g.element(i).unwrap();
    let subgroup = element
        .subgroup()
        .iter()
        .map(|e| e.value.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    rsx! {
      tr {
        td { "{element.value}" }
        td { "{element.order()}" }
        td { "{ subgroup }" }
      }
    }
}
