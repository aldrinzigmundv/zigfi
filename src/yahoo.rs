#![forbid(unsafe_code)]

use chrono::{DateTime, Utc};
use tokio::runtime::Runtime;
use yahoo::YSearchResult;
use yahoo_finance_api as yahoo;

use crate::format;

///Requests ticker's current and previous price based on provided interval
pub fn get(query: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> (f64, f64) {
    let rt = Runtime::new().expect("Failed to start Runtime");
    let yahoo = yahoo::YahooConnector::new();
    let response = rt
        .block_on(yahoo.get_quote_history(query, from, to))
        .expect("Yahoo Finance request failed. Invalid ticker on the watchlist?");
    let quote = response
        .quotes()
        .expect("Failed to process Yahoo Finance Response.");
    let difference = format::reduc(format::prcnt(
        quote
            .last()
            .expect("Failed to process Yahoo Finance Response.")
            .close,
        quote
            .get(0)
            .expect("Failed to process Yahoo Finance Response.")
            .close,
    ));
    (
        format::reduc(
            quote
                .last()
                .expect("Failed to process Yahoo Finance Response.")
                .close,
        ),
        difference,
    )
}

///Returns search results of provided query
pub fn search(query: &str) -> YSearchResult {
    let rt = Runtime::new().expect("Failed to start Runtime");
    let yahoo = yahoo::YahooConnector::new();
    rt.block_on(yahoo.search_ticker(query))
        .expect("Failed to process Yahoo Finance Response.")
}
