use anyhow::{anyhow, bail, Context};
use scraper::{ElementRef, Html};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, trace};
use crate::model::{Day, DayOverview, EditAction, EntryType, ParticipantType, PersonEntry, RosterEntry, RosterEntryType, TimeFrame};

#[derive(Debug, Default)]
pub struct Parser {
    selectors: Selectors,
}

#[derive(Debug)]
struct Selectors {
    table: scraper::Selector,
    tr: scraper::Selector,
    td: scraper::Selector,
    a: scraper::Selector,
    action: scraper::Selector,
    format: scraper::Selector,
    edit_id: scraper::Selector,
    remarks: scraper::Selector,
    checked: scraper::Selector,
    username: scraper::Selector,
}

impl Selectors {
    fn new() -> Self {
        Self {
            table: scraper::Selector::parse("table").unwrap(),
            tr: scraper::Selector::parse("tr").unwrap(),
            td: scraper::Selector::parse("td").unwrap(),
            a: scraper::Selector::parse("a").unwrap(),
            action: scraper::Selector::parse("body > form > input[type=hidden]:nth-child(3)").unwrap(),
            format: scraper::Selector::parse("body > form > input[type=hidden]:nth-child(6)").unwrap(),
            edit_id: scraper::Selector::parse("body > form > input[type=hidden]:nth-child(5)").unwrap(),
            remarks: scraper::Selector::parse("body > form > table:nth-child(1) > tbody > tr:nth-child(7) > td > textarea").unwrap(),
            checked: scraper::Selector::parse("input[checked]").unwrap(),
            username: scraper::Selector::parse("body > p > table:nth-child(3) > tbody > tr:nth-child(1) > td > a").unwrap()
        }
    }
}

impl Default for Selectors {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {

    #[instrument(skip(document))]
    pub fn parse_day(&self, document: String) -> anyhow::Result<Day> {
        let document = scraper::Html::parse_document(&document);
        let roster = self.parse_roster(&document).context("could not parse roster")?;
        let action = self.parse_action(&document).context("could not parse action")?;
        let format = document.select(&self.selectors.format).next()
            .context("could not select format field")?
            .value().attr("value")
            .context("could not select value attribute")?
            .to_string();
        match action {
            EditAction::Add => Ok(Day {
                entries: roster,
                action,
                id: None,
                participant_type: ParticipantType::GliderPilot,
                format,
                remarks: None,
                entry_type: None,
            }),
            EditAction::Edit => {
                let id = document.select(&self.selectors.edit_id).next()
                    .context("could not select edit field")?
                    .value().attr("value")
                    .context("could not select value attribute")?
                    .parse::<i32>()
                    .context("could not parse id")?;
                let remarks = document.select(&self.selectors.remarks).next()
                    .context("could not select remarks field")?
                    .text().collect::<String>();
                let checked = document.select(&self.selectors.checked).next()
                    .context("could not select checked field")?
                    .value().attr("value")
                    .context("could not select value attribute")?
                    .to_string();
                let entry_type = match checked.as_str() {
                    "1" => Some(RosterEntryType::Tentative),
                    "2" => Some(RosterEntryType::Definite),
                    "-1" => Some(RosterEntryType::Unavailable),
                    _ => None,
                };
                Ok(Day {
                    entries: roster,
                    action,
                    id: Some(id),
                    participant_type: ParticipantType::GliderPilot,
                    format,
                    remarks: Some(remarks),
                    entry_type,
                })
            }
        }
    }

    #[instrument(skip(document))]
    pub fn parse_action(&self, document: &Html) -> anyhow::Result<EditAction> {
        let action = document.select(&self.selectors.action).next()
            .context("could not select action field")?;
        let action = action.value().attr("value").unwrap();
        match action {
            "edit" => Ok(EditAction::Edit),
            "add" => Ok(EditAction::Add),
            _ => bail!("unknown action: {}", action),
        }
    }

    #[instrument(skip(document))]
    pub fn parse_roster(&self, document: &Html) -> anyhow::Result<Vec<RosterEntry>> {
        // find 2nd table element
        let table = document.select(&self.selectors.table).nth(2)
            .context("could not select table")?;
        // iterate over trs
        let mut rows = table.select(&self.selectors.tr);
        let mut current_entry_type = RosterEntryType::Definite;
        // skip the first row
        let _ = rows.next();
        // headers have the format "Name (Count)"
        // the first entries are Definite entry types
        // once we reach a row with only one td we know that is the header for the Tentative entries
        // once we again reach a row with only one td we know that is the header for the Unavailable entries
        let mut roster_entries = Vec::new();
        for row in rows {
            let tds = row.select(&self.selectors.td);
            let tds = tds.map(|td| td.text().collect::<String>()).collect::<Vec<_>>();
            if tds.len() == 1 {
                // header row
                if row.text().collect::<String>() == "kein Eintrag" {
                    continue;
                }
                trace!("header row: {:?}", tds);
                if current_entry_type == RosterEntryType::Definite {
                    current_entry_type = RosterEntryType::Tentative;
                } else if current_entry_type == RosterEntryType::Tentative {
                    current_entry_type = RosterEntryType::Unavailable;
                }
            } else {
                // entry row
                trace!("entry row: {:?}", tds);
                let name = tds[0].clone();
                let message = tds[2].clone();
                roster_entries.push(RosterEntry {
                    name,
                    message,
                    entry_type: current_entry_type,
                });
            }
        }

        Ok(roster_entries)
    }

    #[instrument(skip(document))]
    pub fn parse_calendar(&self, document: String) -> anyhow::Result<Vec<DayOverview>> {
        let document = scraper::Html::parse_document(&document);
        // find table element
        let table = document.select(&self.selectors.table).take(1).next()
            .context("could not select table")?;

        // find table rows
        let rows = table.select(&self.selectors.tr);
        // skip the first four rows
        let rows = rows.skip(4);
        let rows = rows.flat_map(|row| self.parse_row(row)).collect::<Vec<_>>();

        // group by day
        let mut grouped_rows: Vec<Vec<TableEntry>> = Vec::new();
        let mut current_day = None;
        for row in rows {
            if let Some(day) = current_day {
                if day == row.date {
                    grouped_rows.last_mut()
                        .context("could not get last row")?
                        .push(row);
                } else {
                    current_day = Some(row.date);
                    grouped_rows.push(vec![row]);
                }
            } else {
                current_day = Some(row.date);
                grouped_rows.push(vec![row]);
            }
        }

        Ok(grouped_rows.into_iter().flat_map(DayOverview::try_from).collect::<Vec<_>>())
    }

    #[instrument(skip(el))]
    fn parse_row(&self, el: ElementRef) -> anyhow::Result<TableEntry> {
        let mut tds = el.select(&self.selectors.td);
        // first td contains date
        let date = tds.next().map(|td| self.parse_date(td))
            .ok_or_else(|| anyhow!("coult not get next td"))?
            .context("could not parse date")?;
        // skip second (kw)
        let _ = tds.next();
        // third td contains count of registered pilots
        let (reg, pot) = parse_registered_pilots(tds.next().context("could not get next td")?);
        // skip the next two
        // holidays & events
        let day_note = tds.next()
            .context("could not get next td")?
            .text().collect::<String>().trim().to_string();
        let day_note = if day_note.is_empty() {
            None
        } else {
            Some(day_note)
        };
        // short note for flight information (eg. winch evening or no motorized flights)
        let note_2 = tds.next()
            .context("could not get next td")?
            .text().collect::<String>().trim().to_string();
        let note_2 = if note_2.is_empty() {
            None
        } else {
            Some(note_2)
        };
        // time frame of entry
        let time_frame = parse_time_frame(tds.next()
            .context("could not get next td")?
        );
        // next is entry type
        let entry_type = parse_entry_type(tds.next()
            .context("could not get next td")?
        );
        // skip 3
        let _ = tds.next();
        let _ = tds.next();
        // longer note
        let note = tds.next()
            .context("could not get next td")?;
        // if note contains an <a> tag, extract its title
        let note_3 = note.select(&self.selectors.a).next().map(|a| a.value()
            .attr("title").map(|v| v.to_string())).flatten();
        // next is name of person corresponding to entry type
        let name = tds.next().context("could not get next td")?.text().collect::<String>().trim().to_string();
        let name = if name.is_empty() {
            None
        } else {
            Some(name)
        };

        Ok(TableEntry {
            date,
            registered_pilots: (reg, pot),
            time_frame,
            entry_type,
            name,
            day_note,
            note_1: note_2,
            note_2: note_3,
        })
    }

    #[instrument(skip(el))]
    fn parse_date(&self, el: ElementRef) -> anyhow::Result<chrono::NaiveDate> {
        // select a tags
        let selected = el.select(&self.selectors.a).nth(1);
        selected
            .map(|el| el.value().attr("name")).flatten()
            .ok_or_else(|| anyhow!("could not get name attribute"))
            .map(|v| chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d").context("could not parse date"))?
            .context("could not parse date")
    }
    
    #[instrument(skip(document))]
    pub fn parse_menu(&self, document: String) -> anyhow::Result<String> {
        let document = scraper::Html::parse_document(&document);
        let username = document.select(&self.selectors.username).next()
            .context("could not select username")?
            .text().collect::<String>().trim().to_string();
        Ok(username)
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
        let date = value.first().unwrap().date;
        let note = value.first().unwrap().day_note.clone();
        let registered_pilots = value.first().unwrap().registered_pilots;
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

#[instrument(skip(el))]
fn parse_entry_type(el: ElementRef) -> EntryType {
    let text = el.text().collect::<String>();
    let text = text.trim();
    match text {
        "FI" => EntryType::FlightInstructor,
        "S" => EntryType::TowingPilot,
        "W" => EntryType::WinchOperator,
        _ => panic!("Unknown entry type: {}", text),
    }
}

#[instrument(skip(el))]
fn parse_registered_pilots(el: ElementRef) -> (u32, u32) {
    let text = el.text().collect::<String>();
    let text = text.trim();
    let text = text.replace("&nbsp;", "");
    if text.is_empty() || text == "SF+" {
        (0, 0)
    } else {
        let mut parts = text.split('(');
        let reg = parts.next().unwrap().trim().parse::<u32>().unwrap();
        let pot = parts.next().unwrap().trim().strip_suffix(')').unwrap().parse::<u32>().unwrap();
        (reg, pot)
    }
}


#[instrument(skip(el))]
fn parse_time_frame(el: ElementRef) -> TimeFrame {
    let text = el.text().collect::<String>();
    let text = text.trim();
    let text = text.replace("&nbsp;", "");
    let mut parts = text.split('-');
    let start = chrono::NaiveTime::parse_from_str(parts.next().unwrap().trim(), "%H:%M").unwrap();
    let end = chrono::NaiveTime::parse_from_str(parts.next().unwrap().trim(), "%H:%M").unwrap();
    (start, end)
}
