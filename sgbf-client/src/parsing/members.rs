use std::fmt::Display;
use itertools::Itertools;
use scraper::{ElementRef, Html};
use crate::model::{Member, Period, Reservation};

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
        let rows = table.select(&self.selectors.tr).skip(6);

        let mut members = vec![];
        for row in rows {

        }

        Ok(members)
    }
}