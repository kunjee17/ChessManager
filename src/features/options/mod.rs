use dioxus::prelude::*;
#[component]
pub fn Options() -> Element {
    rsx! {
        div { class: "p-6",
            div { class: "mb-6",
                h1 { class: "text-3xl font-bold text-primary", "Options" }
                p { class: "text-base-content/70 mt-2",
                    "Configure your tournament management settings"
                }
            }
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4", "Tournament Defaults" }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text", "Default Time Control" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "classical", "Classical (90+30)" }
                                option { value: "rapid", "Rapid (15+10)" }
                                option { value: "blitz", "Blitz (5+3)" }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label",
                                span { class: "label-text", "Default Tournament Format" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "swiss", "Swiss System" }
                                option { value: "roundrobin", "Round Robin" }
                                option { value: "knockout", "Knockout" }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label",
                                span { class: "label-text", "Tiebreak Methods (in order)" }
                            }
                            div { class: "join w-full",
                                select { class: "select select-bordered join-item w-full",
                                    option { "1. Buchholz Cut 1" }
                                    option { "1. Direct Encounter" }
                                    option { "1. Sonneborn-Berger" }
                                }
                            }
                            div { class: "join w-full mt-2",
                                select { class: "select select-bordered join-item w-full",
                                    option { "2. Sonneborn-Berger" }
                                    option { "2. Buchholz Cut 1" }
                                    option { "2. Direct Encounter" }
                                }
                            }
                        }
                    }
                }
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4", "Display Settings" }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text", "Theme" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "light", "Light" }
                                option { value: "dark", "Dark" }
                                option { value: "system", "System" }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label",
                                span { class: "label-text", "Language" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "en", "English" }
                                option { value: "es", "Español" }
                                option { value: "de", "Deutsch" }
                                option { value: "fr", "Français" }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label",
                                span { class: "label-text", "Date Format" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "mdy", "MM/DD/YYYY" }
                                option { value: "dmy", "DD/MM/YYYY" }
                                option { value: "ymd", "YYYY-MM-DD" }
                            }
                        }
                    }
                }
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4", "Pairing Settings" }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text", "Color Allocation Method" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "strict", "Strict Alternation" }
                                option { value: "balanced", "Color Balance" }
                                option { value: "random", "Random with Balance" }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label",
                                span { class: "label-text", "Rating System" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "fide", "FIDE" }
                                option { value: "uscf", "USCF" }
                                option { value: "ecf", "ECF" }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label",
                                span { class: "label-text", "Pairing Algorithm" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "dutch", "Dutch System (FIDE)" }
                                option { value: "burstein", "Burstein System" }
                                option { value: "simple", "Simple Pairing" }
                            }
                        }
                    }
                }
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4", "Export Settings" }
                        div { class: "form-control",
                            label { class: "label",
                                span { class: "label-text", "Default Export Format" }
                            }
                            select { class: "select select-bordered w-full",
                                option { value: "pdf", "PDF" }
                                option { value: "csv", "CSV" }
                                option { value: "pgn", "PGN" }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label cursor-pointer",
                                span { class: "label-text", "Auto-export results after each round" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-primary",
                                }
                            }
                        }
                        div { class: "form-control mt-4",
                            label { class: "label cursor-pointer",
                                span { class: "label-text", "Include ratings in exports" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-primary",
                                    checked: true,
                                }
                            }
                        }
                    }
                }
            }
            div { class: "mt-6 flex justify-end",
                button { class: "btn btn-primary", "Save Changes" }
            }
        }
    }
}
