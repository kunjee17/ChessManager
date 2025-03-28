use dioxus::prelude::*;
#[component]
pub fn Players() -> Element {
    rsx! {
        div { class: "p-6",
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-3xl font-bold text-primary", "Players" }
                div { class: "flex gap-4",
                    div { class: "form-control",
                        input {
                            class: "input input-bordered",
                            placeholder: "Search players...",
                            r#type: "text",
                        }
                    }
                    button { class: "btn btn-primary", "Add Player" }
                }
            }
            div { class: "card bg-base-200",
                div { class: "card-body",
                    div { class: "overflow-x-auto",
                        table { class: "table table-zebra w-full",
                            thead {
                                tr {
                                    th { "Name" }
                                    th { "Rating" }
                                    th { "Title" }
                                    th { "Tournaments Played" }
                                    th { "Current Tournament" }
                                    th { "Actions" }
                                }
                            }
                            tbody {
                                tr {
                                    td {
                                        div { class: "flex items-center gap-3",
                                            div { class: "avatar placeholder",
                                                div { class: "bg-neutral text-neutral-content rounded-full w-8",
                                                    span { "JD" }
                                                }
                                            }
                                            "John Doe"
                                        }
                                    }
                                    td { "2200" }
                                    td { "FM" }
                                    td { "15" }
                                    td {
                                        span { class: "badge badge-info", "Winter Championship" }
                                    }
                                    td {
                                        div { class: "flex gap-2",
                                            button { class: "btn btn-sm btn-info", "Profile" }
                                            button { class: "btn btn-sm btn-ghost", "History" }
                                        }
                                    }
                                }
                                tr {
                                    td {
                                        div { class: "flex items-center gap-3",
                                            div { class: "avatar placeholder",
                                                div { class: "bg-neutral text-neutral-content rounded-full w-8",
                                                    span { "AS" }
                                                }
                                            }
                                            "Alice Smith"
                                        }
                                    }
                                    td { "1850" }
                                    td { "-" }
                                    td { "8" }
                                    td {
                                        span { class: "badge badge-ghost", "None" }
                                    }
                                    td {
                                        div { class: "flex gap-2",
                                            button { class: "btn btn-sm btn-info", "Profile" }
                                            button { class: "btn btn-sm btn-ghost", "History" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex justify-center mt-4",
                        div { class: "join",
                            button { class: "join-item btn", "«" }
                            button { class: "join-item btn btn-active", "1" }
                            button { class: "join-item btn", "2" }
                            button { class: "join-item btn", "3" }
                            button { class: "join-item btn", "»" }
                        }
                    }
                }
            }
        }
    }
}
