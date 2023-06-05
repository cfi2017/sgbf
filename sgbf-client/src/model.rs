use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RosterEntry {
    pub name: String,
    pub message: String,
    pub entry_type: RosterEntryType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum RosterEntryType {
    Definite,
    Tentative,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Day {
    pub date: chrono::NaiveDate,
    pub registered_pilots: Stats,
    pub entries: Vec<PersonEntry>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub definitive: u32,
    pub tentative: u32,
}

impl From<(u32, u32)> for Stats {
    fn from((definitive, tentative): (u32, u32)) -> Self {
        Self {
            definitive,
            tentative,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonEntry {
    pub time_frame: TimeFrame,
    pub name: String,
    pub entry_type: EntryType,
    pub note_1: Option<String>,
    pub note_2: Option<String>,
}

pub type TimeFrame = (chrono::NaiveTime, chrono::NaiveTime);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EntryType {
    #[serde(rename = "FI")]
    FlightInstructor,
    #[serde(rename = "S")]
    TowingPilot,
    #[serde(rename = "W")]
    WinchOperator,
}
