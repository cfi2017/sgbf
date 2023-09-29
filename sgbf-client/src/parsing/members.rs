use std::fmt::Display;
use itertools::Itertools;
use scraper::{ElementRef, Html};
use crate::model::{Addresses, Member, Period, Reservation};
use crate::model::EditAction::Add;

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
    pub fn parse(&self, document: &Html) -> anyhow::Result<Vec<Member>> {
        let table = document.select(&self.selectors.table).next().unwrap();
        let rows = table.select(&self.selectors.tr).skip(18).chunks(12);

        let mut members = vec![];
        for mut row in &rows {
            let row = row.next().unwrap();
            let tds = row.select(&self.selectors.td).collect::<Vec<_>>();
            let name = super::get_text(tds[0]).join("");
            let address = super::get_text(tds[2]).join("\n");
            let phone = super::get_text(tds[5]);
            let fax = super::get_text(tds[8]);
            let mobile = super::get_text(tds[11]);
            let mail = super::get_text(tds[14]);
            let member = Member {
                name,
                address: Some(address),
                private: Addresses {
                    phone: phone.get(0).map(|s| s.to_string()),
                    email: mail.get(0).map(|s| s.to_string()),
                    mobile: mobile.get(0).map(|s| s.to_string()),
                },
                office: Addresses {
                    phone: fax.get(1).map(|s| s.to_string()),
                    email: mail.get(1).map(|s| s.to_string()),
                    mobile: mobile.get(1).map(|s| s.to_string()),
                },
            };
            members.push(member);
        }

        Ok(members)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {
        // tests/data/reservations.html
        let document = include_str!("../../tests/data/members.html");
        let document = scraper::Html::parse_document(document);
        let parser = super::Parser::default();
        let result = parser.parse(&document);
        assert!(result.is_ok());
    }
}
