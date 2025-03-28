use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct DaisyErrorProps {
    message: String,
}
#[component]
pub fn DaisyError(props: DaisyErrorProps) -> Element {
    rsx! {
        div { class: "alert alert-error shadow-lg",
            div {
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    class: "stroke-current shrink-0 h-6 w-6",
                    fill: "none",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                    }
                }
                span { "{props.message}" }
            }
        }
    }
}
