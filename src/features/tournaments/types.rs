use crate::entity::tournament;
use anyhow::Result;
use chrono::NaiveDateTime;
use dioxus::prelude::FormValue;
use sea_orm::ActiveValue::{NotSet, Set};
use std::collections::HashMap;
use validator::Validate;
#[derive(Debug, Validate, Clone, Default)]
pub struct TournamentForm {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub total_rounds: i32,
    pub max_players: i32,
}
impl TournamentForm {
    pub fn from_hashmap(values: HashMap<String, FormValue>) -> Result<Self> {
        let name = values
            .get("name")
            .ok_or(anyhow::anyhow!("Name is required"))?
            .as_value();
        let description = values
            .get("description")
            .ok_or(anyhow::anyhow!("Description is required"))?
            .as_value();
        let start_date_str = values
            .get("start_date")
            .ok_or(anyhow::anyhow!("Start date is required"))?
            .as_value();
        let end_date_str = values
            .get("end_date")
            .ok_or(anyhow::anyhow!("End date is required"))?
            .as_value();
        let start_date = format!("{}:00", start_date_str).parse::<NaiveDateTime>()?;
        let end_date = format!("{}:00", end_date_str).parse::<NaiveDateTime>()?;
        let total_rounds = values
            .get("total_rounds")
            .ok_or(anyhow::anyhow!("Total rounds is required"))?
            .as_value()
            .parse::<i32>()?;
        let max_players = values
            .get("max_players")
            .ok_or(anyhow::anyhow!("Max players is required"))?
            .as_value()
            .parse::<i32>()?;
        Ok(TournamentForm {
            id: None,
            name,
            description,
            start_date,
            end_date,
            total_rounds,
            max_players,
        })
    }
    pub fn to_active_model(&self) -> tournament::ActiveModel {
        tournament::ActiveModel {
            id: NotSet,
            name: Set(self.name.clone()),
            description: Set(self.description.clone()),
            start_date: Set(self.start_date.and_utc()),
            end_date: Set(self.end_date.and_utc()),
            total_rounds: Set(self.total_rounds as i32),
            max_players: Set(self.max_players as i32),
            current_round: Set(0),
            status: Set("Active".to_string()),
            current_players: Set(0),
            time_control: Set(String::new()),
            format: Set(String::new()),
            tiebreak_methods: Set(String::new()),
            color_allocation: Set(String::new()),
            pairing_algorithm: Set(String::new()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }
}
