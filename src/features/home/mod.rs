use crate::Route;
use dioxus::prelude::*;
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "min-h-screen",
            div { class: "hero bg-base-200 py-12",
                div { class: "hero-content text-center",
                    div { class: "max-w-2xl",
                        h1 { class: "text-5xl font-bold mb-8", "Chess Tournament Manager" }
                        p { class: "text-xl mb-8",
                            "Organize and manage chess tournaments with ease. From Swiss pairings to time management, we've got you covered."
                        }
                        Link {
                            to: Route::Tournaments {},
                            class: "btn btn-primary btn-lg",
                            "Get Started"
                        }
                    }
                }
            }
            div { class: "container mx-auto px-4 py-8",
                h2 { class: "text-2xl font-bold mb-6", "Quick Actions" }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    Link {
                        to: Route::Tournaments {},
                        class: "card bg-primary text-primary-content hover:bg-primary-focus transition-colors",
                        div { class: "card-body",
                            h3 { class: "card-title",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-6 w-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z",
                                    }
                                }
                                "Create Tournament"
                            }
                            p { "Start a new chess tournament with customizable settings" }
                        }
                    }
                    Link {
                        to: Route::Players {},
                        class: "card bg-secondary text-secondary-content hover:bg-secondary-focus transition-colors",
                        div { class: "card-body",
                            h3 { class: "card-title",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-6 w-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z",
                                    }
                                }
                                "Manage Players"
                            }
                            p { "Add and manage player profiles and ratings" }
                        }
                    }
                    Link {
                        to: Route::TimeCalculator {},
                        class: "card bg-accent text-accent-content hover:bg-accent-focus transition-colors",
                        div { class: "card-body",
                            h3 { class: "card-title",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "h-6 w-6",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    stroke: "currentColor",
                                    path {
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        "stroke-width": "2",
                                        d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z",
                                    }
                                }
                                "Calculate Time"
                            }
                            p { "Plan your tournament schedule with our time calculator" }
                        }
                    }
                }
            }
            div { class: "container mx-auto px-4 py-8",
                h2 { class: "text-2xl font-bold mb-6", "Features" }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                    div { class: "card bg-base-200",
                        div { class: "card-body",
                            h3 { class: "card-title", "Swiss System" }
                            p { "Automatic pairing using the Swiss system with customizable parameters" }
                            ul { class: "mt-4 space-y-2",
                                li { class: "flex items-center gap-2",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "h-5 w-5 text-success",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M5 13l4 4L19 7",
                                        }
                                    }
                                    "Fair pairings"
                                }
                                li { class: "flex items-center gap-2",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "h-5 w-5 text-success",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M5 13l4 4L19 7",
                                        }
                                    }
                                    "Color balance"
                                }
                            }
                        }
                    }
                    div { class: "card bg-base-200",
                        div { class: "card-body",
                            h3 { class: "card-title", "Time Management" }
                            p { "Comprehensive time control options for different tournament formats" }
                            ul { class: "mt-4 space-y-2",
                                li { class: "flex items-center gap-2",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "h-5 w-5 text-success",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M5 13l4 4L19 7",
                                        }
                                    }
                                    "Multiple time controls"
                                }
                                li { class: "flex items-center gap-2",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "h-5 w-5 text-success",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M5 13l4 4L19 7",
                                        }
                                    }
                                    "Schedule planning"
                                }
                            }
                        }
                    }
                    div { class: "card bg-base-200",
                        div { class: "card-body",
                            h3 { class: "card-title", "Player Management" }
                            p { "Complete player profile and rating management system" }
                            ul { class: "mt-4 space-y-2",
                                li { class: "flex items-center gap-2",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "h-5 w-5 text-success",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M5 13l4 4L19 7",
                                        }
                                    }
                                    "Rating tracking"
                                }
                                li { class: "flex items-center gap-2",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        class: "h-5 w-5 text-success",
                                        fill: "none",
                                        "viewBox": "0 0 24 24",
                                        stroke: "currentColor",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "stroke-width": "2",
                                            d: "M5 13l4 4L19 7",
                                        }
                                    }
                                    "Performance history"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
