use dioxus::prelude::*;
use group::{Finite, GroupElement, ZnMul, ZnMulElement};
mod const_utils;
mod group;
mod utils;

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

        Group { n }

    }
}

#[component]
pub fn Group(n: ReadOnlySignal<usize>) -> Element {
    const N: usize = 32usize;
    rsx! {
        div {
            h1 {
              "Z mod {N} Z"
            }
            table {
              tr {
                th { "Element" }
                th { "Order" }
              }
              for i in 1..N  {
                if ZnMulElement::<N>::new(i).is_some() {
                  tr {
                    td { "{i}" }
                    td { "{ZnMulElement::<N>::new(i).unwrap().order()}" }
                  }
                }
              }
            }
        }
    }
}
