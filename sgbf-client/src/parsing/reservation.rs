use std::fmt::Display;
use itertools::Itertools;
use scraper::{ElementRef, Html};
use crate::model::{Period, Reservation};

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
    pub fn parse(&self, document: &Html) -> anyhow::Result<Vec<Reservation>> {
        // first table
        let table = document.select(&self.selectors.table).next().unwrap();
        // println!("table: {:?}", table.text().collect::<Vec<_>>());
        // iterate over children
        let rows = table.select(&self.selectors.tr).skip(2);
        // iterate over rows in groups of three
        let rows = rows.chunks(5);

        let mut reservations = vec![];
        for chunk in &rows {
            if let Some((plane, _, _, info, _)) = chunk.collect_tuple() {
                // only rows with nested table
                let table = plane.select(&self.selectors.table).next().unwrap();
                // plane information, date when reservation was made, who made the reservation
                // a[name] contains the id
                let id = table.select(&self.selectors.a).next().unwrap();
                let id = id.value().attr("name").unwrap();
                let plane_data = get_text(table);
                let (plane, reservation_date, who) = (plane_data[0].clone(), plane_data[1].clone(), plane_data[2].clone());
                let reservation_date = chrono::NaiveDate::parse_from_str(&reservation_date, "%d.%m.%Y").unwrap();
                let info = get_text(info);
                let (period, comments) = if info.len() == 1 {
                    // single day, no comments (period)
                    (parse_single_day(&info[0])?, vec![])
                } else if info.len() == 2 {
                    // either single day with comments or multiple days without comments
                    if let Ok(period) = parse_multiple_days(&info[0], &info[1]) {
                        // multiple days without comments
                        (period, vec![])
                    } else {
                        // single day with comments
                        (parse_single_day(&info[0])?, info[1..].to_vec())
                    }
                } else {
                    // either single day with comments or multiple days with comments
                    if let Ok(period) = parse_multiple_days(&info[0], &info[1]) {
                        // multiple days with comments
                        (period, info[2..].to_vec())
                    } else {
                        // single day with comments
                        (parse_single_day(&info[0])?, info[1..].to_vec())
                    }
                };
                let reservation = Reservation {
                    id: id.parse().unwrap(),
                    plane,
                    created_at: reservation_date,
                    reserved_by: who,
                    period,
                    comments,
                };
                reservations.push(reservation);
            } else {
                println!("truncating error");
            }
        }
        Ok(reservations)
    }

}

fn get_text(table: ElementRef) -> Vec<String> {
    table.text().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<_>>()
}

// parse single day with format "16.07.2023\u{a0}\u{a0}11:00\u{a0}-\u{a0}18:00"
fn parse_single_day(value: &str) -> anyhow::Result<Period> {
    let mut parts = value.split_whitespace();
    let (date, from, _, to) = (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap());
    let from = format!("{} {}", date, from);
    let to = format!("{} {}", date, to);
    let from = chrono::NaiveDateTime::parse_from_str(&from, "%d.%m.%Y %H:%M")?;
    let to = chrono::NaiveDateTime::parse_from_str(&to, "%d.%m.%Y %H:%M")?;
    Ok(Period { from, to })
}

// from format: 17.07.2023\u{a0}\u{a0}09:00\u{a0}-
// to format: 21.07.2023\u{a0}\u{a0}20:00
fn parse_multiple_days(from: &str, to: &str) -> anyhow::Result<Period> {
    // sanitize
    let from = from.trim_end_matches('-');
    // whitespace
    let from = from.split_whitespace().collect::<Vec<_>>().join(" ");
    let to = to.split_whitespace().collect::<Vec<_>>().join(" ");
    let from = chrono::NaiveDateTime::parse_from_str(from.as_str(), "%d.%m.%Y %H:%M")?;
    let to = chrono::NaiveDateTime::parse_from_str(to.as_str(), "%d.%m.%Y %H:%M")?;
    Ok(Period { from, to })
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {
        // tests/data/reservations.html
        let document = include_str!("../../tests/data/reservations.html");
        let document = scraper::Html::parse_document(document);
        let parser = super::Parser::default();
        let result = parser.parse(&document);
        assert!(result.is_ok());
    }
}