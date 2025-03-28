pub mod active;
pub mod list;
pub mod new;
pub mod past;
pub mod queries;
pub mod types;
pub mod upcoming;
use crate::Route;
use dioxus::prelude::*;
use list::TournamentsList;
#[component]
pub fn Tournaments() -> Element {
    rsx! {
        div { class: "p-6",
            div { class: "flex justify-between items-center mb-6",
                h1 { class: "text-3xl font-bold text-primary", "Tournaments" }
                Link { class: "btn btn-primary", to: Route::NewTournament {}, "Create New Tournament" }
            }
            TournamentsList {}
        }
    }
}
