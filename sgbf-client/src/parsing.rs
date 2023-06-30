mod calendar;
mod day;
mod menu;
mod reservation;

use anyhow::{Context};

use serde::{Deserialize, Serialize};
use tracing::{instrument};
use crate::model::{Day, DayOverview, EntryType, PersonEntry, Reservation, TimeFrame};

#[derive(Debug, Default)]
pub struct Parser {
    day_parser: day::Parser,
    calendar_parser: calendar::Parser,
    menu_parser: menu::Parser,
    reservation_parser: reservation::Parser,
}

impl Parser {

    #[instrument(skip(document))]
    pub fn parse_day(&self, document: String) -> anyhow::Result<Day> {
        let document = scraper::Html::parse_document(&document);
        self.day_parser.parse(&document)
    }

    #[instrument(skip(document))]
    pub fn parse_calendar(&self, document: String) -> anyhow::Result<Vec<DayOverview>> {
        let document = scraper::Html::parse_document(&document);
        self.calendar_parser.parse(&document)
    }

    #[instrument(skip(document))]
    pub fn parse_menu(&self, document: String) -> anyhow::Result<String> {
        let document = scraper::Html::parse_document(&document);
        self.menu_parser.parse(&document)
    }

    #[instrument(skip(document))]
    pub fn parse_reservations(&self, document: String) -> anyhow::Result<Vec<Reservation>> {
        let document = scraper::Html::parse_document(&document);
        self.reservation_parser.parse(&document)
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableEntry {
    date: chrono::NaiveDate,
    registered_pilots: (u32, u32),
    time_frame: TimeFrame,
    entry_type: EntryType,
    name: Option<String>,
    day_note: Option<String>,
    note_1: Option<String>,
    note_2: Option<String>,
}

impl TryFrom<Vec<TableEntry>> for DayOverview {
    type Error = anyhow::Error;

    #[instrument(skip(value))]
    fn try_from(value: Vec<TableEntry>) -> anyhow::Result<Self> {
        let first = value.first().context("couldn't fetch first entry")?;
        let date = first.date;
        let note = first.day_note.clone();
        let registered_pilots = first.registered_pilots;
        // filter out entries without name
        let entries = value.into_iter().filter_map(|entry| {
            if let Some(name) = entry.name {
                Some(PersonEntry {
                    time_frame: entry.time_frame,
                    name,
                    entry_type: entry.entry_type,
                    note_1: entry.note_1,
                    note_2: entry.note_2,
                })
            } else {
                None
            }
        }).collect::<Vec<_>>();
        Ok(DayOverview {
            date,
            registered_pilots: registered_pilots.into(),
            entries,
            note,
        })
    }
}
