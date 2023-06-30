use std::fmt::Display;
use chrono::NaiveDate;
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
    pub reservations: Option<Vec<Reservation>>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Period {
    pub from: chrono::NaiveDateTime,
    pub to: chrono::NaiveDateTime,
}

impl From<NaiveDate> for Period {
    fn from(date: NaiveDate) -> Self {
        let date_end = date.and_hms_opt(23, 59, 59);
        let date_start = date.and_hms_opt(0, 0, 0);
        Self {
            from: date_start.unwrap(),
            to: date_end.unwrap(),
        }
    }
}

pub trait Overlaps<T> {
    fn overlaps(&self, other: &T) -> bool;
}

impl Overlaps<NaiveDate> for Period {
    fn overlaps(&self, other: &NaiveDate) -> bool {
        let other = Period::from(*other);
        self.from <= other.to && other.from <= self.to
    }
}

impl Overlaps<Period> for Period {
    fn overlaps(&self, other: &Period) -> bool {
        self.from <= other.to && other.from >= self.to
    }
}

impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.from, self.to)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reservation {
    pub id: i32,
    pub period: Period,
    pub plane: String,
    pub reserved_by: String,
    pub created_at: chrono::NaiveDate,
    pub comments: Vec<String>
}
