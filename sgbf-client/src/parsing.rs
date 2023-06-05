use scraper::ElementRef;
use serde::{Deserialize, Serialize};
use tracing::{debug, trace};
use crate::model::{Day, EntryType, PersonEntry, RosterEntry, RosterEntryType, TimeFrame};

pub fn parse_roster(document: String) -> Vec<RosterEntry> {
    let document = scraper::Html::parse_document(&document);
    // find 2nd table element
    let table = document.select(&scraper::Selector::parse("table").unwrap()).nth(2).unwrap();
    // iterate over trs
    let selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();
    let mut rows = table.select(&selector);
    debug!("found rows: {:?}", rows.clone().count());
    let mut current_entry_type = RosterEntryType::Definite;
    // skip the first row
    let _ = rows.next();
    // headers have the format "Name (Count)"
    // the first entries are Definite entry types
    // once we reach a row with only one td we know that is the header for the Tentative entries
    // once we again reach a row with only one td we know that is the header for the Unavailable entries
    let mut roster_entries = Vec::new();
    for row in rows {
        let tds = row.select(&td_selector);
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

    roster_entries
}

pub fn parse_calendar(document: String) -> Vec<Day> {
    let document = scraper::Html::parse_document(&document);
    // find table element
    let table = document.select(&scraper::Selector::parse("table").unwrap()).take(1).next().unwrap();

    // find table rows
    let selector = scraper::Selector::parse("tr").unwrap();
    let rows = table.select(&selector);
    // skip the first four rows
    let rows = rows.skip(4);
    let rows = rows.map(parse_row).collect::<Vec<_>>();

    // group by day
    let mut grouped_rows: Vec<Vec<TableEntry>> = Vec::new();
    let mut current_day = None;
    for row in rows {
        if let Some(day) = current_day {
            if day == row.date {
                grouped_rows.last_mut().unwrap().push(row);
            } else {
                current_day = Some(row.date);
                grouped_rows.push(vec![row]);
            }
        } else {
            current_day = Some(row.date);
            grouped_rows.push(vec![row]);
        }
    }

    grouped_rows.into_iter().map(Day::from).collect::<Vec<_>>()
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

impl From<Vec<TableEntry>> for Day {
    fn from(value: Vec<TableEntry>) -> Self {
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
        Day {
            date,
            registered_pilots: registered_pilots.into(),
            entries,
            note,
        }
    }
}

fn parse_row(el: ElementRef) -> TableEntry {
    let td_selector = scraper::Selector::parse("td").unwrap();
    let mut tds = el.select(&td_selector);
    // first td contains date
    let date = parse_date(tds.next().unwrap());
    // skip second (kw)
    let _ = tds.next();
    // third td contains count of registered pilots
    let (reg, pot) = parse_registered_pilots(tds.next().unwrap());
    // skip the next two
    // holidays & events
    let day_note = tds.next().unwrap().text().collect::<String>().trim().to_string();
    let day_note = if day_note.is_empty() {
        None
    } else {
        Some(day_note)
    };
    // short note for flight information (eg. winch evening or no motorized flights)
    let note_2 = tds.next().unwrap().text().collect::<String>().trim().to_string();
    let note_2 = if note_2.is_empty() {
        None
    } else {
        Some(note_2)
    };
    // time frame of entry
    let time_frame = parse_time_frame(tds.next().unwrap());
    // next is entry type
    let entry_type = parse_entry_type(tds.next().unwrap());
    // skip 3
    let _ = tds.next();
    let _ = tds.next();
    // longer note
    let note = tds.next().unwrap();
    // if note contains an <a> tag, extract its title
    let note_3 = note.select(&scraper::Selector::parse("a").unwrap()).next().map(|a| a.value().attr("title").unwrap().to_string());
    // next is name of person corresponding to entry type
    let name = tds.next().unwrap().text().collect::<String>().trim().to_string();
    let name = if name.is_empty() {
        None
    } else {
        Some(name)
    };

    TableEntry {
        date,
        registered_pilots: (reg, pot),
        time_frame,
        entry_type,
        name,
        day_note: day_note,
        note_1: note_2,
        note_2: note_3,
    }
}

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

fn parse_date(el: ElementRef) -> chrono::NaiveDate {
    // select a tags
    let a_selector = scraper::Selector::parse("a").unwrap();
    let mut a_tags = el.select(&a_selector);
    // second a tag contains date
    let el = a_tags.nth(1).unwrap();
    let date = el.value().attr("name").unwrap();
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

fn parse_time_frame(el: ElementRef) -> TimeFrame {
    let text = el.text().collect::<String>();
    let text = text.trim();
    let text = text.replace("&nbsp;", "");
    let mut parts = text.split('-');
    let start = chrono::NaiveTime::parse_from_str(parts.next().unwrap().trim(), "%H:%M").unwrap();
    let end = chrono::NaiveTime::parse_from_str(parts.next().unwrap().trim(), "%H:%M").unwrap();
    (start, end)
}
