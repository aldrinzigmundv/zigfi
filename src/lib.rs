#![forbid(unsafe_code)]

use chrono;
use confy;
use crossterm::{
    event::{poll, read, Event, KeyCode},
    style::Color,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration, vec};
use tokio::runtime::Runtime;
use yahoo_finance_api as yahoo;

mod format;
mod terminal;

//zigfi configuration structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    greenisup: bool,
    watchlists: HashMap<String, Vec<String>>,
}

//Required for Config structs in confy crate
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            greenisup: true,
            watchlists: HashMap::new(),
        }
    }
}

//Sets up default configuration if not available
pub fn startup() {
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    if cfg.watchlists.contains_key("default") {
    } else {
        cfg.watchlists.insert(
            "default".to_string(),
            vec![
                "XMR-USD".to_string(),
                "BTC-USD".to_string(),
                "GC=F".to_string(),
                "SI=F".to_string(),
            ],
        );
        confy::store("zigfi", cfg).expect("Failed to save new configuration.");
    }
}

//Displays watchlist
pub fn display(query: &str, interval: &str) {
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    let rt = Runtime::new().expect("Failed to start Runtime");
    let yahoo = yahoo::YahooConnector::new();
    let watchlist = cfg
        .watchlists
        .get(query)
        .expect("Failed to load watchlist.");
    if watchlist.is_empty() {
        terminal::write("Watchlist is empty. Press q to quit.");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    } else {
        let now = chrono::Utc::now();
        let mut back = chrono::Duration::days(2);
        if interval.eq_ignore_ascii_case("1mo") {
            back = chrono::Duration::weeks(4);
        } else if interval.eq_ignore_ascii_case("1y") {
            back = chrono::Duration::weeks(52);
        }
        'outer: loop {
            for ticker in watchlist.iter() {
                let response = rt
                    .block_on(yahoo.get_quote_history(ticker, now - back, now))
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
                terminal::write_within_space(ticker, 10);
                terminal::set_color(Color::Yellow);
                terminal::write_within_space(
                    format::reduc(
                        quote
                            .last()
                            .expect("Failed to process Yahoo Finance Response.")
                            .close,
                    )
                    .to_string()
                    .as_ref(),
                    20,
                );
                terminal::reset_color();
                if difference.is_sign_positive() && cfg.greenisup {
                    terminal::set_color(Color::Green);
                } else if difference.is_sign_negative() && cfg.greenisup {
                    terminal::set_color(Color::Red);
                } else if difference.is_sign_positive() && !cfg.greenisup {
                    terminal::set_color(Color::Red);
                } else {
                    terminal::set_color(Color::Green);
                }
                terminal::write(difference.to_string().as_ref());
                terminal::write("%");
                terminal::write("   ");
                terminal::reset_color();
                terminal::skip_line();
            }
            terminal::skip_line();
            terminal::write("Press q to quit.");
            terminal::reset_cursor();
            if poll(Duration::from_millis(500)).expect("Terminal error.") {
                let event = read().expect("Terminal error.");
                if event == Event::Key(KeyCode::Char('q').into()) {
                    break 'outer;
                }
            }
        }
    }
}

//Shows search results for provided query
pub fn search(query: &str) {
    let rt = Runtime::new().expect("Failed to start Runtime");
    let yahoo = yahoo::YahooConnector::new();
    let resp = rt
        .block_on(yahoo.search_ticker(query))
        .expect("Failed to process Yahoo Finance Response.");
    let mut captured = false;
    loop {
        if captured == false {
            for item in &resp.quotes {
                terminal::write_within_space(item.symbol.as_ref(), 18);
                terminal::write_then_nextline(item.short_name.as_ref());
            }
            captured = true;
        }
        terminal::skip_line();
        terminal::write("Search results displayed. Press q to quit.");
        let event = read().expect("Terminal error.");
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }
}

//Creates a new watchlist
pub fn new(watchlist: &str, tickers: Vec<String>) {
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    if cfg.watchlists.contains_key(watchlist) {
        terminal::write_then_nextline("Watchlist provided already exist and will be overwritten.");
        terminal::write_then_nextline("Do you wish to continue? (y/n)");
        loop {
            let event = read().expect("Terminal error.");
            if event == Event::Key(KeyCode::Char('y').into()) {
                new_continuation(cfg, watchlist, tickers);
                break;
            } else if event == Event::Key(KeyCode::Char('n').into()) {
                break;
            }
        }
    } else {
        new_continuation(cfg, watchlist, tickers);
    }
}

//Continues creating new watchlist once duplicate has been cleared if there'll be at new()
fn new_continuation(mut cfg: Config, watchlist: &str, tickers: Vec<String>) {
    let rt = Runtime::new().expect("Failed to start Runtime");
    let yahoo = yahoo::YahooConnector::new();
    loop {
        let mut verified_tickers: Vec<String> = vec![];
        for ticker in tickers.iter() {
            let _ = rt
                .block_on(yahoo.get_latest_quotes(ticker, "1d"))
                .expect("Yahoo Finance request failed. Invalid ticker on the watchlist?");
            verified_tickers.push(ticker.to_string());
        }
        if verified_tickers.is_empty() {
            cfg.watchlists.insert(watchlist.to_string(), vec![]);
            confy::store("zigfi", cfg).expect("Failed to save.");
            terminal::write_then_nextline("Empty watchlist created");
            terminal::write("Press q to quit...");
        } else {
            terminal::write_then_nextline("Watchlist created");
            terminal::write("Press q to quit...");
            cfg.watchlists
                .insert(watchlist.to_string(), verified_tickers);
            confy::store("zigfi", cfg).expect("Terminal error.");
        }
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
        break;
    }
}

//Adds ticker/s to watchlist
pub fn add(watchlist: &str, tickers: Vec<String>) {
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    let rt = Runtime::new().expect("Failed to start Runtime");
    let yahoo = yahoo::YahooConnector::new();
    let mut verified_tickers: Vec<String> = vec![];
    for ticker in tickers.iter() {
        let _ = rt
            .block_on(yahoo.get_latest_quotes(ticker, "1d"))
            .expect("Yahoo Finance request failed. Invalid ticker on the watchlist?");
        verified_tickers.push(ticker.to_string());
    }
    let mut clone = cfg
        .watchlists
        .get(watchlist)
        .expect("Internal error.")
        .clone();
    for ticker in verified_tickers {
        clone.push(ticker);
    }
    cfg.watchlists.insert(watchlist.to_string(), clone);
    confy::store("zigfi", cfg).expect("Failed to save.");
    terminal::write_then_nextline("Ticker/s has been added to the watchlist.");
    terminal::write("Press q to quit...");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

//Removes ticker/s to watchlist
pub fn remove(watchlist: &str, tickers: Vec<String>) {
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    let mut clone = cfg
        .watchlists
        .get(watchlist)
        .expect("Failed to load watchlist.")
        .clone();
    let mut for_removal: Vec<usize> = vec![];
    for ticker in tickers.iter() {
        let index = cfg
            .watchlists
            .get(watchlist)
            .expect("Failed to load watchlist.")
            .iter()
            .position(|x| x == ticker)
            .expect("Failed to find ticker/s. Invalid ticker/s?");
        for_removal.push(index);
    }
    for index in for_removal {
        clone.remove(index);
    }
    cfg.watchlists.insert(watchlist.to_string(), clone);
    confy::store("zigfi", cfg).expect("Failed to save.");
    terminal::write_then_nextline("Ticker/s has been removed from the watchlist.");
    terminal::write("Press q to quit...");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

//Deletes watchlist
pub fn delete(query: &str) {
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    if cfg.watchlists.contains_key(query) {
        cfg.watchlists.remove(query);
        confy::store("zigfi", cfg).expect("Failed to save.");
        terminal::write_then_nextline("Watchlist has been deleted.");
        terminal::write("Press q to quit...");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    } else {
        terminal::write_then_nextline("Watchlist does not exist.");
        terminal::write("Press q to quit...");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    }
}

//Displays commands available
pub fn help() {
    let help = vec![
        "zigarg - List of Commands",
        "",
        "Commands",
        "zigarg (shows \"default\" watchlist)",
        "zigarg new <watchlist name> <optional: ticker/s>",
        "zigarg show <watchlist name> <optional: interval> (interval can be \"1d\", \"1mo\" or \"1y\")",
        "zigarg delete <watchlist name>",
        "zigarg add <watchlist name> <ticker/s>",
        "zigarg remove <watchlist name> <ticker/s>",
        "zigarg search <name of asset>",
        "zigarg list (lists saved watchlist/s)",
        "zigarg colorswap (swaps Green and Red for some East Asian users)",
        "zigarg help",
        "",
        "Developed by Aldrin Zigmund Cortez Velasco",
        "",
        "Press q to quit...",
    ];
    terminal::write_multiline(help);
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

//Lists watchlists
pub fn list() {
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    for watchlist in cfg.watchlists.keys() {
        terminal::write_then_nextline(watchlist);
    }
    terminal::skip_line();
    terminal::write_then_nextline("Existing watchlist/s displayed.");
    terminal::write("Press q to quit...");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

pub fn colorswap() {
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    if cfg.greenisup {
        cfg.greenisup = false;
    } else {
        cfg.greenisup = true;
    }
    confy::store("zigfi", cfg).expect("Failed to save new configuration.");
    terminal::write("Green and red swapped. Press q to quit...");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}
