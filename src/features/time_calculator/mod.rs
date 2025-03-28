use dioxus::prelude::*;
#[component]
pub fn TimeCalculator() -> Element {
    rsx! {
        div { class: "p-6",
            div { class: "mb-6",
                h1 { class: "text-3xl font-bold text-primary", "Tournament Time Calculator" }
                p { class: "text-base-content/70 mt-2",
                    "Calculate the estimated duration of your chess tournament based on time controls and number of rounds."
                }
            }
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title", "Time Control Settings" }
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text", "Initial Time (minutes)" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "number",
                                placeholder: "90",
                                value: "90",
                            }
                        }
                        div { class: "form-control w-full mt-4",
                            label { class: "label",
                                span { class: "label-text", "Increment per move (seconds)" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "number",
                                placeholder: "30",
                                value: "30",
                            }
                        }
                        div { class: "mt-6",
                            h3 { class: "font-semibold mb-2", "Common Time Controls" }
                            div { class: "flex flex-wrap gap-2",
                                button { class: "btn btn-sm", "Blitz (5+3)" }
                                button { class: "btn btn-sm", "Rapid (15+10)" }
                                button { class: "btn btn-sm", "Classical (90+30)" }
                            }
                        }
                    }
                }
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title", "Tournament Settings" }
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text", "Number of Rounds" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "number",
                                placeholder: "7",
                                value: "7",
                            }
                        }
                        div { class: "form-control w-full mt-4",
                            label { class: "label",
                                span { class: "label-text", "Number of Players" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "number",
                                placeholder: "32",
                                value: "32",
                            }
                        }
                        div { class: "form-control w-full mt-4",
                            label { class: "label",
                                span { class: "label-text", "Break Between Rounds (minutes)" }
                            }
                            input {
                                class: "input input-bordered w-full",
                                r#type: "number",
                                placeholder: "15",
                                value: "15",
                            }
                        }
                    }
                }
            }
            div { class: "card bg-primary text-primary-content mt-6",
                div { class: "card-body",
                    h2 { class: "card-title", "Estimated Tournament Duration" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-4 mt-4",
                        div { class: "stat",
                            div { class: "stat-title", "Maximum Game Duration" }
                            div { class: "stat-value", "3h 30m" }
                            div { class: "stat-desc", "Per round" }
                        }
                        div { class: "stat",
                            div { class: "stat-title", "Total Playing Time" }
                            div { class: "stat-value", "24h 30m" }
                            div { class: "stat-desc", "Not including breaks" }
                        }
                        div { class: "stat",
                            div { class: "stat-title", "Total Tournament Time" }
                            div { class: "stat-value", "26h 15m" }
                            div { class: "stat-desc", "Including breaks" }
                        }
                    }
                }
            }
            div { class: "mt-6 space-y-4",
                div { class: "alert alert-info",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "stroke-current shrink-0 h-6 w-6",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                    }
                    span {
                        "These calculations assume maximum game duration. Actual tournament time may vary."
                    }
                }
                div { class: "alert alert-warning",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "stroke-current shrink-0 h-6 w-6",
                        fill: "none",
                        "viewBox": "0 0 24 24",
                        path {
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                            d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
                        }
                    }
                    span { "Consider adding extra buffer time for potential delays and long games." }
                }
            }
        }
    }
}
