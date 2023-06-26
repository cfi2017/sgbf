use anyhow::{anyhow, Context};
use scraper::{ElementRef, Html};
use tracing::instrument;
use crate::model::{DayOverview, EntryType, TimeFrame};
use crate::parsing::TableEntry;

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
}

impl Selectors {
    fn new() -> Self {
        Self {
            table: scraper::Selector::parse("table").unwrap(),
            tr: scraper::Selector::parse("tr").unwrap(),
            td: scraper::Selector::parse("td").unwrap(),
            a: scraper::Selector::parse("a").unwrap(),
        }
    }
}

impl Default for Selectors {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {

    pub fn parse(&self, document: &Html) -> anyhow::Result<Vec<DayOverview>> {
        // find table element
        let table = document.select(&self.selectors.table).take(1).next()
            .context("could not select table")?;

        // find table rows
        let rows = table.select(&self.selectors.tr);
        // skip the first four rows
        let rows = rows.skip(4);
        let rows = rows.filter(|row| row.children().count() > 3).flat_map(|row| self.parse_row(row)).collect::<Vec<_>>();

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

    fn parse_date(&self, el: ElementRef) -> anyhow::Result<chrono::NaiveDate> {
        // select a tags
        let selected = el.select(&self.selectors.a).nth(1);
        selected
            .map(|el| el.value().attr("name")).flatten()
            .ok_or_else(|| anyhow!("could not get name attribute"))
            .map(|v| chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d").context("could not parse date"))?
            .context("could not parse date")
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
