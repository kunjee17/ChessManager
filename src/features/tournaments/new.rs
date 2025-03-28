use super::{queries::save_tournament, types::TournamentForm};
use crate::{components::daisy_error::DaisyError, Route};
use dioxus::prelude::*;
use std::collections::HashMap;
#[component]
pub fn NewTournament() -> Element {
    let mut values: Signal<HashMap<String, FormValue>> = use_signal(HashMap::new);
    let mut errors = use_signal(Vec::new);
    let mut success = use_signal(String::new);
    let navigator = use_navigator();
    rsx! {
        div { class: "container mx-auto p-4 max-w-3xl",
            div { class: "card bg-base-200 shadow-xl",
                div { class: "card-body",
                    h2 { class: "card-title text-2xl font-bold mb-6", "Create New Tournament" }
                    if !success().is_empty() {
                        div { class: "alert alert-success mb-4",
                            span { "{success}" }
                        }
                    }
                    if !errors().is_empty() {
                        DaisyError { message: errors().join(", ") }
                    }
                    form {
                        class: "space-y-6",
                        onsubmit: move |ev: Event<FormData>| async move {
                            ev.prevent_default();
                            errors.set(Vec::new());
                            success.set(String::new());
                            match TournamentForm::from_hashmap(values()) {
                                Ok(form) => {
                                    println!("Form: {:#?}", form);
                                    let res = save_tournament(form).await;
                                    match res {
                                        Ok(tournament) => {
                                            success
                                                .set(
                                                    format!(
                                                        "Tournament created successfully! ID: {}",
                                                        tournament.id,
                                                    ),
                                                );
                                            navigator.push(Route::Tournaments {});
                                        }
                                        Err(e) => {
                                            errors.set(vec![e.to_string()]);
                                        }
                                    }
                                }
                                Err(e) => {
                                    errors.set(vec![e.to_string()]);
                                }
                            }
                        },
                        oninput: move |ev| {
                            values.set(ev.values());
                        },
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Tournament Name" }
                            }
                            input {
                                r#type: "text",
                                class: "input input-bordered w-full",
                                name: "name",
                                placeholder: "Enter tournament name",
                            }
                        }
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Description" }
                            }
                            textarea {
                                class: "textarea textarea-bordered h-32 w-full",
                                name: "description",
                                placeholder: "Enter tournament description",
                            }
                        }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                            div { class: "form-control w-full",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Start Date" }
                                }
                                input {
                                    r#type: "datetime-local",
                                    class: "input input-bordered w-full",
                                    name: "start_date",
                                }
                            }
                            div { class: "form-control w-full",
                                label { class: "label",
                                    span { class: "label-text font-medium", "End Date" }
                                }
                                input {
                                    r#type: "datetime-local",
                                    class: "input input-bordered w-full",
                                    name: "end_date",
                                }
                            }
                        }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                            div { class: "form-control w-full",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Total Rounds" }
                                }
                                input {
                                    r#type: "number",
                                    class: "input input-bordered w-full",
                                    name: "total_rounds",
                                    min: "1",
                                    max: "20",
                                    value: "7",
                                }
                            }
                            div { class: "form-control w-full",
                                label { class: "label",
                                    span { class: "label-text font-medium", "Max Players" }
                                }
                                input {
                                    r#type: "number",
                                    class: "input input-bordered w-full",
                                    name: "max_players",
                                    min: "2",
                                    max: "100",
                                    value: "32",
                                }
                            }
                        }
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Time Control" }
                            }
                            input {
                                r#type: "text",
                                class: "input input-bordered w-full",
                                name: "time_control",
                                placeholder: "e.g., 90+30",
                            }
                        }
                        div { class: "form-control w-full",
                            label { class: "label",
                                span { class: "label-text font-medium", "Format" }
                            }
                            select {
                                class: "select select-bordered w-full",
                                name: "format",
                                option { value: "Swiss", "Swiss System" }
                                option { value: "Round Robin", "Round Robin" }
                                option { value: "Knockout", "Knockout" }
                            }
                        }
                        div { class: "form-control mt-8",
                            button {
                                class: "btn btn-primary w-full",
                                r#type: "submit",
                                "Create Tournament"
                            }
                        }
                    }
                }
            }
        }
    }
}
