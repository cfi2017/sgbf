use anyhow::{bail, Context};
use scraper::Html;
use tracing::{instrument, trace};
use crate::model::{Day, EditAction, ParticipantType, RosterEntry, RosterEntryType};

#[derive(Debug, Default)]
pub struct Parser {
    selectors: Selectors,
}

#[derive(Debug)]
struct Selectors {
    table: scraper::Selector,
    tr: scraper::Selector,
    td: scraper::Selector,
    action: scraper::Selector,
    format: scraper::Selector,
    edit_id: scraper::Selector,
    remarks: scraper::Selector,
    checked: scraper::Selector,
}

impl Selectors {
    fn new() -> Self {
        Self {
            table: scraper::Selector::parse("table").unwrap(),
            tr: scraper::Selector::parse("tr").unwrap(),
            td: scraper::Selector::parse("td").unwrap(),
            action: scraper::Selector::parse("body > form > input[type=hidden]:nth-child(3)").unwrap(),
            format: scraper::Selector::parse("body > form > input[type=hidden]:nth-child(6)").unwrap(),
            edit_id: scraper::Selector::parse("body > form > input[type=hidden]:nth-child(5)").unwrap(),
            remarks: scraper::Selector::parse("body > form > table:nth-child(1) > tbody > tr:nth-child(7) > td > textarea").unwrap(),
            checked: scraper::Selector::parse("input[checked]").unwrap(),
        }
    }
}

impl Default for Selectors {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {

    pub fn parse(&self, document: &Html) -> anyhow::Result<Day> {
        let roster = self.parse_roster(document).context("could not parse roster")?;
        let action = self.parse_action(document).context("could not parse action")?;
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

}