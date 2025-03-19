use dioxus::prelude::*;

#[component]
pub fn Options() -> Element {
    rsx! {
        div { class: "p-6",
            // Header
            div { class: "mb-6",
                h1 { class: "text-3xl font-bold text-primary",
                    "Options"
                }
                p { class: "text-base-content/70 mt-2",
                    "Configure your tournament management settings"
                }
            }

            // Settings Grid
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                // Tournament Defaults
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4",
                            "Tournament Defaults"
                        }

                        // Default Time Control
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

                        // Default Tournament Format
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

                        // Tiebreak Settings
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

                // Display Settings
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4",
                            "Display Settings"
                        }

                        // Theme
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

                        // Language
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

                        // Date Format
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

                // Pairing Settings
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4",
                            "Pairing Settings"
                        }

                        // Color Allocation
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

                        // Rating System
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

                        // Pairing Algorithm
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

                // Export Settings
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title mb-4",
                            "Export Settings"
                        }

                        // Export Format
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

                        // Auto-Export
                        div { class: "form-control mt-4",
                            label { class: "label cursor-pointer",
                                span { class: "label-text", "Auto-export results after each round" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-primary"
                                }
                            }
                        }

                        // Include Ratings
                        div { class: "form-control mt-4",
                            label { class: "label cursor-pointer",
                                span { class: "label-text", "Include ratings in exports" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-primary",
                                    checked: true
                                }
                            }
                        }
                    }
                }
            }

            // Save Button
            div { class: "mt-6 flex justify-end",
                button { class: "btn btn-primary",
                    "Save Changes"
                }
            }
        }
    }
}
