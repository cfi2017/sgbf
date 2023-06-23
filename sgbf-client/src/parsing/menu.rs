use anyhow::Context;
use scraper::Html;
use tracing::instrument;

#[derive(Debug, Default)]
pub struct Parser {
    selectors: Selectors,
}

#[derive(Debug)]
struct Selectors {
    username: scraper::Selector,
}

impl Selectors {
    fn new() -> Self {
        Self {
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

    pub fn parse(&self, document: &Html) -> anyhow::Result<String> {
        let username = document.select(&self.selectors.username).next()
            .context("could not select username")?
            .text().collect::<String>().trim().to_string();
        Ok(username)
    }
}