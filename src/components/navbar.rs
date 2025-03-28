use crate::Route;
use dioxus::prelude::*;
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "drawer lg:drawer-open",
            input {
                id: "main-drawer",
                r#type: "checkbox",
                class: "drawer-toggle",
            }
            div { class: "drawer-content flex flex-col",
                div { class: "w-full navbar bg-base-300",
                    div { class: "flex-none lg:hidden",
                        label {
                            class: "btn btn-square btn-ghost",
                            r#for: "main-drawer",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                class: "inline-block w-6 h-6 stroke-current",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "M4 6h16M4 12h16M4 18h16",
                                }
                            }
                        }
                    }
                    div { class: "flex-1",
                        Link {
                            to: Route::Home {},
                            class: "btn btn-ghost text-xl",
                            "Chess Manager"
                        }
                    }
                    div { class: "flex-none",
                        label { class: "swap swap-rotate btn btn-ghost btn-circle",
                            input {
                                r#type: "checkbox",
                                class: "theme-controller",
                                value: "synthwave",
                            }
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "swap-off h-6 w-6 fill-current",
                                "viewBox": "0 0 24 24",
                                path { d: "M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" }
                            }
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "swap-on h-6 w-6 fill-current",
                                "viewBox": "0 0 24 24",
                                path { d: "M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" }
                            }
                        }
                    }
                }
                div { class: "p-4", Outlet::<Route> {} }
            }
            div { class: "drawer-side",
                label { class: "drawer-overlay", r#for: "main-drawer" }
                ul { class: "menu p-4 w-64 h-full bg-base-200 text-base-content",
                    li { class: "menu-title lg:hidden",
                        span { "Chess Manager" }
                    }
                    li {
                        Link {
                            to: Route::Home {},
                            class: "flex items-center gap-2",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6",
                                }
                            }
                            "Home"
                        }
                    }
                    li {
                        Link {
                            to: Route::Tournaments {},
                            class: "flex items-center gap-2",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
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
                            "Tournaments"
                        }
                    }
                    li {
                        Link {
                            to: Route::Players {},
                            class: "flex items-center gap-2",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
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
                            "Players"
                        }
                    }
                    li {
                        Link {
                            to: Route::TimeCalculator {},
                            class: "flex items-center gap-2",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
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
                            "Time Calculator"
                        }
                    }
                    li {
                        Link {
                            to: Route::Options {},
                            class: "flex items-center gap-2",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "h-5 w-5",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                stroke: "currentColor",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z",
                                }
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "stroke-width": "2",
                                    d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z",
                                }
                            }
                            "Options"
                        }
                    }
                }
            }
        }
    }
}
