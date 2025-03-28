use dioxus::prelude::*;
#[component]
pub fn PrimaryLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner loading-lg text-primary" }
    }
}
#[component]
pub fn SecondaryLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner loading-lg text-secondary" }
    }
}
#[component]
pub fn AccentLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner text-accent" }
    }
}
#[component]
pub fn NeutralLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner text-neutral" }
    }
}
#[component]
pub fn InfoLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner text-info" }
    }
}
#[component]
pub fn SuccessLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner text-success" }
    }
}
#[component]
pub fn WarningLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner text-warning" }
    }
}
#[component]
pub fn ErrorLoading() -> Element {
    rsx! {
        span { class: "loading loading-spinner text-error" }
    }
}
