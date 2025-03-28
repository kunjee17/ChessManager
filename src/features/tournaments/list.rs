use crate::features::tournaments::active::ActiveTournaments;
use crate::features::tournaments::past::PastTournaments;
use crate::features::tournaments::upcoming::UpcomingTournaments;
use dioxus::prelude::*;
#[component]
pub fn TournamentsList() -> Element {
    rsx! {
        div { class: "grid gap-4",
            ActiveTournaments {}
            UpcomingTournaments {}
            PastTournaments {}
        }
    }
}
