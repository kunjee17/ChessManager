use dioxus::prelude::*;

#[component]
pub fn Tournaments() -> Element {
    rsx! {
        div { class: "p-6",
            // Header Section
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-3xl font-bold text-primary",
                    "Tournaments"
                }
                button { class: "btn btn-primary",
                    "Create New Tournament"
                }
            }

            // Tournament List
            div { class: "grid gap-4",
                // Active Tournaments Section
                div { class: "card bg-base-200",
                    div { class: "card-body",
                        h2 { class: "card-title text-xl mb-4",
                            "Active Tournaments"
                        }
                        div { class: "overflow-x-auto",
                            table { class: "table table-zebra w-full",
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "Players" }
                                        th { "Rounds" }
                                        th { "Status" }
                                        th { "Actions" }
                                    }
                                }
                                tbody {
                                    tr {
                                        td { "Winter Championship 2024" }
                                        td { "32/32" }
                                        td { "Round 3/7" }
                                        td {
                                            span { class: "badge badge-success",
                                                "In Progress"
                                            }
                                        }
                                        td {
                                            div { class: "flex gap-2",
                                                button { class: "btn btn-sm btn-info",
                                                    "View"
                                                }
                                                button { class: "btn btn-sm btn-warning",
                                                    "Manage"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Upcoming Tournaments Section
                div { class: "card bg-base-200 mt-6",
                    div { class: "card-body",
                        h2 { class: "card-title text-xl mb-4",
                            "Upcoming Tournaments"
                        }
                        div { class: "overflow-x-auto",
                            table { class: "table table-zebra w-full",
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "Date" }
                                        th { "Format" }
                                        th { "Registration" }
                                        th { "Actions" }
                                    }
                                }
                                tbody {
                                    tr {
                                        td { "Spring Open 2024" }
                                        td { "Apr 15, 2024" }
                                        td { "Swiss (7 rounds)" }
                                        td {
                                            span { class: "badge badge-info",
                                                "12/32 Registered"
                                            }
                                        }
                                        td {
                                            div { class: "flex gap-2",
                                                button { class: "btn btn-sm btn-primary",
                                                    "Register"
                                                }
                                                button { class: "btn btn-sm btn-ghost",
                                                    "Details"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Past Tournaments Section
                div { class: "card bg-base-200 mt-6",
                    div { class: "card-body",
                        h2 { class: "card-title text-xl mb-4",
                            "Past Tournaments"
                        }
                        div { class: "overflow-x-auto",
                            table { class: "table table-zebra w-full",
                                thead {
                                    tr {
                                        th { "Name" }
                                        th { "Date" }
                                        th { "Winner" }
                                        th { "Players" }
                                        th { "Actions" }
                                    }
                                }
                                tbody {
                                    tr {
                                        td { "Fall Classic 2023" }
                                        td { "Nov 10, 2023" }
                                        td { "GM John Doe" }
                                        td { "64" }
                                        td {
                                            button { class: "btn btn-sm btn-ghost",
                                                "View Results"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
