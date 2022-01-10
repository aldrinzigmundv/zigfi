#![forbid(unsafe_code)]

use confy;
use crossterm::{
    event::{poll, read, Event, KeyCode},
    style::Color,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration, vec};

mod format;
mod output;
mod yahoo;

///zigfi configuration structure
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    greenisup: bool,
    watchlists: HashMap<String, Vec<String>>,
}

///Required for Config structs in confy crate
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            greenisup: true,
            watchlists: HashMap::new(),
        }
    }
}

///Sets up default configuration if not available
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

///Displays watchlist on the terminal
pub fn display(query: &str, interval: &str) {
    output::setup();
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    let watchlist = cfg
        .watchlists
        .get(query)
        .expect("Failed to load watchlist.");
    if watchlist.is_empty() {
        output::write("Watchlist is empty. Press q to quit.");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    } else {
        let (from, to) = format::get_time(interval);
        'outer: loop {
            for ticker in watchlist.iter() {
                let (quote, difference) = yahoo::get(ticker, from, to);
                output::write_within_space(ticker, 10);
                output::set_color(Color::Yellow);
                output::write_within_space(quote.to_string().as_ref(), 20);
                output::reset_color();
                if difference.is_sign_positive() && cfg.greenisup {
                    output::set_color(Color::Green);
                } else if difference.is_sign_negative() && cfg.greenisup {
                    output::set_color(Color::Red);
                } else if difference.is_sign_positive() && !cfg.greenisup {
                    output::set_color(Color::Red);
                } else {
                    output::set_color(Color::Green);
                }
                output::write(difference.to_string().as_ref());
                output::write("%");
                output::write("   ");
                output::reset_color();
                output::skip_line();
            }
            output::skip_line();
            output::write("Press q to quit.");
            output::reset_cursor();
            if poll(Duration::from_millis(500)).expect("Terminal error.") {
                let event = read().expect("Terminal error.");
                if event == Event::Key(KeyCode::Char('q').into()) {
                    break 'outer;
                }
            }
        }
    }
}

///Prints watchlist as text for piping
pub fn print(query: &str, interval: &str) {
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    let watchlist = cfg
        .watchlists
        .get(query)
        .expect("Failed to load watchlist.");
    if watchlist.is_empty() {
        panic!("Watchlist empty");
    } else {
        let (from, to) = format::get_time(interval);
        for ticker in watchlist.iter() {
            let (quote, difference) = yahoo::get(ticker, from, to);
            println!(
                "{} {} {}%",
                ticker,
                quote.to_string(),
                difference.to_string()
            );
        }
    }
}

///Prints watchlist as json for piping
pub fn print_json(query: &str, interval: &str) {
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    let watchlist = cfg
        .watchlists
        .get(query)
        .expect("Failed to load watchlist.");
    if watchlist.is_empty() {
        panic!("Watchlist empty");
    } else {
        let (from, to) = format::get_time(interval);
        for ticker in watchlist.iter() {
            let (quote, difference) = yahoo::get(ticker, from, to);
            println!(
                "{{\"ticker\":\"{}\",\"price\":{},\"difference\":{}%}}",
                ticker,
                quote.to_string(),
                difference.to_string()
            );
        }
    }
}

///Shows search results for provided query
pub fn search(query: &str) {
    output::setup();
    let resp = yahoo::search(query);
    let mut captured = false;
    loop {
        if captured == false {
            for item in &resp.quotes {
                output::write_within_space(item.symbol.as_ref(), 18);
                output::write_then_nextline(item.short_name.as_ref());
            }
            captured = true;
        }
        output::skip_line();
        output::write("Search results displayed. Press q to quit.");
        let event = read().expect("Terminal error.");
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }
}

///Creates a new watchlist
pub fn new(watchlist: &str, tickers: Vec<String>) {
    output::setup();
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    if cfg.watchlists.contains_key(watchlist) {
        output::write_then_nextline("Watchlist provided already exist and will be overwritten.");
        output::write_then_nextline("Do you wish to continue? (y/n)");
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

///Continues creating new watchlist once duplicate has been cleared if there'll be at new()
fn new_continuation(mut cfg: Config, watchlist: &str, tickers: Vec<String>) {
    let (from, now) = format::get_time("1d");
    loop {
        let mut verified_tickers: Vec<String> = vec![];
        for ticker in tickers.iter() {
            let _ = yahoo::get(ticker, from, now);
            verified_tickers.push(ticker.to_string());
        }
        if verified_tickers.is_empty() {
            cfg.watchlists.insert(watchlist.to_string(), vec![]);
            confy::store("zigfi", cfg).expect("Failed to save.");
            output::write_then_nextline("Empty watchlist created");
            output::write("Press q to quit...");
        } else {
            output::write_then_nextline("Watchlist created");
            output::write("Press q to quit...");
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

///Adds ticker/s to watchlist
pub fn add(watchlist: &str, tickers: Vec<String>) {
    output::setup();
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    let (from, now) = format::get_time("1d");
    let mut verified_tickers: Vec<String> = vec![];
    for ticker in tickers.iter() {
        let _ = yahoo::get(ticker, from, now);
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
    output::write_then_nextline("Ticker/s has been added to the watchlist.");
    output::write("Press q to quit...");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

///Removes ticker/s to watchlist
pub fn remove(watchlist: &str, tickers: Vec<String>) {
    output::setup();
    if tickers.is_empty() {
        output::write_then_nextline("You did not provide any ticker.");
        output::skip_line();
        output::write_then_nextline("Use delete to delete a watchlist.");
        output::write_then_nextline("Use remove to remove ticker/s from a watchlist.");
        output::skip_line();
        output::write("Operation aborted. Press q to quit...");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    } else {
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
        output::write_then_nextline("Ticker/s has been removed from the watchlist.");
        output::write("Press q to quit...");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    }
}

///Deletes an existing watchlist
pub fn delete(query: &str) {
    output::setup();
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    if cfg.watchlists.contains_key(query) {
        cfg.watchlists.remove(query);
        confy::store("zigfi", cfg).expect("Failed to save.");
        output::write_then_nextline("Watchlist has been deleted.");
        output::write("Press q to quit...");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    } else {
        output::write_then_nextline("Watchlist does not exist.");
        output::write("Press q to quit...");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    }
}

///Displays commands available
pub fn help() {
    output::setup();
    let help = vec![
        "zigfi - List of Commands",
        "",
        "Commands",
        "zigfi (shows \"default\" watchlist)",
        "zigfi new <watchlist name> <optional: ticker/s>",
        "zigfi show <watchlist name> <optional: interval> (interval can be \"1d\", \"1mo\" or \"1y\")",
        "zigfi delete <watchlist name>",
        "zigfi add <watchlist name> <ticker/s>",
        "zigfi remove <watchlist name> <ticker/s>",
        "zigfi search <name of asset>",
        "zigfi list (lists saved watchlist/s)",
        "zigfi colorswap (swaps Green and Red for some East Asian users)",
        "zigfi help",
        "",
        "\"zigfi show\" supports piping. Default output is string. Add \"--json\" for JSON.",
        "",
        "Developed by Aldrin Zigmund Cortez Velasco",
        "",
        "Press q to quit...",
    ];
    output::write_multiline(help);
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

///Lists watchlists
pub fn list() {
    output::setup();
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    for watchlist in cfg.watchlists.keys() {
        output::write_then_nextline(watchlist);
    }
    output::skip_line();
    output::write_then_nextline("Existing watchlist/s displayed.");
    output::write("Press q to quit...");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

///Swaps red and green for some East Asian users
pub fn colorswap() {
    output::setup();
    let mut cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    if cfg.greenisup {
        cfg.greenisup = false;
    } else {
        cfg.greenisup = true;
    }
    confy::store("zigfi", cfg).expect("Failed to save new configuration.");
    output::write("Green and red swapped. Press q to quit...");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}
