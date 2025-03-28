use crate::components::daisy_error::DaisyError;
use crate::components::loading::PrimaryLoading;
use crate::entity::tournament;
use crate::features::tournaments::queries::get_past_tournaments;
use anyhow::Result;
use dioxus::prelude::*;
#[component]
pub fn PastTournaments() -> Element {
    let res: Resource<Result<Vec<tournament::Model>>> = use_resource(move || get_past_tournaments());
    rsx! {
        div { class: "card bg-base-200 mt-6",
            div { class: "card-body",
                h2 { class: "card-title text-xl mb-4", "Past Tournaments" }
                if let Some(tournaments) = &*res.read() {
                    match tournaments {
                        Ok(tournaments) => {
                            if tournaments.is_empty() {
                                rsx! {
                                    div { class: "flex items-center justify-center py-4 text-gray-500",
                                        svg {
                                            xmlns: "http://www.w3.org/2000/svg",
                                            class: "h-5 w-5 mr-2",
                                            fill: "none",
                                            view_box: "0 0 24 24",
                                            stroke: "currentColor",
                                            path {
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                stroke_width: "2",
                                                d: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2",
                                            }
                                        }
                                        span { "No past tournaments found" }
                                    }
                                }
                            } else {
                                rsx! {
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
                                                {
                                                    tournaments
                                                        .iter()
                                                        .map(|tournament| {
                                                            rsx! {
                                                                tr { key: "{tournament.id}",
                                                                    td { "{tournament.name.clone()}" }
                                                                    td { "{tournament.end_date.to_string()}" }
                                                                    td { "TBD" }
                                                                    td { "{tournament.current_players}" }
                                                                    td {
                                                                        button { class: "btn btn-sm btn-ghost", "View Results" }
                                                                    }
                                                                }
                                                            }
                                                        })
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => rsx! {
                            DaisyError { message: format!("Failed to fetch tournaments: {}", err) }
                        },
                    }
                } else {
                    PrimaryLoading {}
                }
            }
        }
    }
}
