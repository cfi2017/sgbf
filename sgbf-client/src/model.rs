use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub entries: Vec<RosterEntry>,
    pub action: EditAction,
    pub id: Option<i32>,
    pub participant_type: ParticipantType,
    pub format: String,
    pub remarks: Option<String>,
    pub entry_type: Option<RosterEntryType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditAction {
    Edit,
    Add,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantType {
    #[serde(rename = "participant_sf")]
    GliderPilot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntry {
    pub name: String,
    pub message: String,
    pub entry_type: RosterEntryType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Copy)]
#[serde(rename_all = "PascalCase")]
pub enum RosterEntryType {
    Definite,
    Tentative,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayOverview {
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
#[serde(rename_all = "camelCase")]
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
    #[serde(rename = "FlightInstructor")]
    FlightInstructor,
    #[serde(rename = "TowingPilot")]
    TowingPilot,
    #[serde(rename = "WinchOperator")]
    WinchOperator,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub name: String,
    pub address: Option<String>,
    pub private: Addresses,
    pub office: Addresses,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Addresses {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
}
