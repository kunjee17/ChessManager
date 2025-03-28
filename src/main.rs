use crate::components::navbar::Navbar;
use crate::db::db::get_db_connection;
use crate::features::{
    home::Home, options::Options, players::Players, time_calculator::TimeCalculator,
    tournaments::{new::NewTournament, Tournaments},
};
use dioxus::{logger::tracing, prelude::*};
use migration::{Migrator, MigratorTrait};
mod components;
mod db;
mod entity;
mod features;
mod utils;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/tournaments")]
    Tournaments {},
    #[route("/tournaments/new")]
    NewTournament {},
    #[route("/players")]
    Players {},
    #[route("/options")]
    Options {},
    #[route("/timecalc")]
    TimeCalculator {},
}
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
async fn init_db() {
    let db = get_db_connection().await.unwrap();
    let _ = Migrator::up(&db, None).await;
}
fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    spawn(async move {
        init_db().await;
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
