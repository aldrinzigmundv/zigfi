use std::{collections::HashMap, ops::Deref, time::Duration, vec};

use confy;
use serde::{Deserialize, Serialize};

use crossterm::{
    cursor::{position, Hide, MoveTo, MoveToNextLine},
    event::{poll, read, Event, KeyCode},
    style::{Color, SetForegroundColor},
    terminal, ExecutableCommand, Result,
};
use std::io::{stdout, Write};

use tokio::runtime::Runtime;
use yahoo::YahooConnector;
use yahoo_finance_api as yahoo;

use chrono;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    greenisup: bool,
    watchlists: HashMap<String, Vec<String>>,
}

/// `Config` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            greenisup: true,
            watchlists: HashMap::new(),
        }
    }
}

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

pub fn start_ui() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(Hide)?;
    stdout.execute(MoveTo(0, 0)).expect("Terminal error.");
    Ok(())
}

pub fn display(rt: Runtime, yahoo: YahooConnector, cfg: Config, query: &str, interval: &str) {
    let watchlist = cfg.watchlists.get(query).expect("Failed to load watchlist.");
    if watchlist.is_empty() {
        stdout()
            .write("Watchlist is empty. Press q to quit.".as_bytes())
            .expect("Terminal error.");
        stdout().flush().expect("Terminal error.");
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
                let quote = response.quotes().expect("Failed to process Yahoo Finance Response.");
                let difference = reduc(prcnt(
                    quote.last().expect("Failed to process Yahoo Finance Response.").close,
                    quote.get(0).expect("Failed to process Yahoo Finance Response.").close,
                ));
                stdout().write(ticker.as_bytes()).expect("Terminal error.");
                let (mut space, _) = position().expect("Terminal error.");
                space = 10_u16 - space;
                for _ in 0..space {
                    stdout().write(" ".as_bytes()).expect("Terminal error.");
                }
                stdout().execute(SetForegroundColor(Color::Yellow)).expect("Terminal error.");
                stdout()
                    .write(reduc(quote.last().expect("Failed to process Yahoo Finance Response.").close).to_string().as_bytes())
                    .expect("Terminal error.");
                stdout().execute(SetForegroundColor(Color::Reset)).expect("Terminal error.");
                let (mut space, _) = position().expect("Terminal error.");
                space = 20_u16 - space;
                for _ in 0..space {
                    stdout().write(" ".as_bytes()).expect("Terminal error.");
                }
                if difference.is_sign_positive() && cfg.greenisup {
                    stdout().execute(SetForegroundColor(Color::Green)).expect("Terminal error.");
                } else {
                    stdout().execute(SetForegroundColor(Color::Red)).expect("Terminal error.");
                }
                stdout().write(difference.to_string().as_bytes()).expect("Terminal error.");
                stdout().write("%".as_bytes()).expect("Terminal error.");
                for _ in 0..3 {
                    stdout().write(" ".as_bytes()).expect("Terminal error.");
                }
                stdout().execute(SetForegroundColor(Color::Reset)).expect("Terminal error.");
                stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
            }
            stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
            stdout().write("Press q to quit.".as_bytes()).expect("Terminal error.");
            stdout().execute(MoveTo(0, 0)).expect("Terminal error.");
            stdout().flush().expect("Terminal error.");
            if poll(Duration::from_millis(500)).expect("Terminal error.") {
                let event = read().expect("Terminal error.");
                if event == Event::Key(KeyCode::Char('q').into()) {
                    break 'outer;
                }
            }
        }
    }
    if watchlist.is_empty() {
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    }
}

pub fn search(rt: Runtime, yahoo: YahooConnector, query: &str) {
    let resp = rt.block_on(yahoo.search_ticker(query)).expect("Failed to process Yahoo Finance Response.");
    let mut captured = false;
    loop {
        if captured == false {
            for item in &resp.quotes {
                stdout().write(item.deref().symbol.as_bytes()).expect("Terminal error.");
                let (mut space, _) = position().expect("Terminal error.");
                space = 18_u16 - space;
                for _ in 0..space {
                    stdout().write(" ".as_bytes()).expect("Terminal error.");
                }
                stdout().write(item.deref().short_name.as_bytes()).expect("Terminal error.");
                stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
            }
            captured = true;
        }
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        stdout().write("Search results displayed. Press q to quit.".as_bytes()).expect("Terminal error.");
        stdout().flush().expect("Terminal error.");
        let event = read().expect("Terminal error.");
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }
}

pub fn create(
    rt: Runtime,
    yahoo: YahooConnector,
    mut cfg: Config,
    watchlist: &str,
    tickers: Vec<&String>,
) {
    if cfg.watchlists.contains_key(watchlist) {
        stdout()
            .write("Watchlist provided already exist and will be overwritten.".as_bytes())
            .expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        stdout()
            .write("Do you wish to continue? (y/n)".as_bytes())
            .expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        stdout().flush().expect("Terminal error.");
        loop {
            let event = read().expect("Terminal error.");
            if event == Event::Key(KeyCode::Char('n').into()) {
                panic!("Operation aborted!");
            } else if event == Event::Key(KeyCode::Char('y').into()) {
                break;
            }
        }
    }
    let mut verified_tickers: Vec<String> = vec![];
    for ticker in tickers.iter().copied() {
        let _ = rt.block_on(yahoo.get_latest_quotes(ticker, "1d")).expect("Yahoo Finance request failed. Invalid ticker on the watchlist?");
        verified_tickers.push(ticker.to_string());
    }
    if verified_tickers.is_empty() {
        stdout()
            .write("Empty watchlist created".as_bytes())
            .expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        stdout().write("Press q to quit...".as_bytes()).expect("Terminal error.");
        stdout().flush().expect("Terminal error.");
        cfg.watchlists.insert(watchlist.to_string(), vec![]);
        confy::store("zigfi", cfg).expect("Failed to save.");
    } else {
        stdout().write("Watchlist created".as_bytes()).expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        stdout().write("Press q to quit...".as_bytes()).expect("Terminal error.");
        stdout().flush().expect("Terminal error.");
        cfg.watchlists
            .insert(watchlist.to_string(), verified_tickers);
        confy::store("zigfi", cfg).expect("Terminal error.");
    }
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

pub fn add(
    rt: Runtime,
    yahoo: YahooConnector,
    mut cfg: Config,
    watchlist: &str,
    tickers: Vec<&String>,
) {
    let mut verified_tickers: Vec<String> = vec![];
    for ticker in tickers.iter().copied() {
        let _ = rt.block_on(yahoo.get_latest_quotes(ticker, "1d")).expect("Yahoo Finance request failed. Invalid ticker on the watchlist?");
        verified_tickers.push(ticker.to_string());
    }
    let mut clone = cfg.watchlists.get(watchlist).expect("Internal error.").clone();
    for ticker in verified_tickers {
        clone.push(ticker);
    }
    cfg.watchlists.insert(watchlist.to_string(), clone);
    confy::store("zigfi", cfg).expect("Failed to save.");
    stdout()
        .write("Ticker/s has been added to the watchlist.".as_bytes())
        .expect("Terminal error.");
    stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
    stdout().write("Press q to quit...".as_bytes()).expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

pub fn remove(mut cfg: Config, watchlist: &str, tickers: Vec<&String>) {
    let mut clone = cfg.watchlists.get(watchlist).expect("Failed to load watchlist.").clone();
    let mut for_removal: Vec<usize> = vec![];
    for ticker in tickers.iter().copied() {
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
    stdout()
        .write("Ticker/s has been removed from the watchlist.".as_bytes())
        .expect("Terminal error.");
    stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
    stdout().write("Press q to quit...".as_bytes()).expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

pub fn delete(mut cfg: Config, query: &str) {
    if cfg.watchlists.contains_key(query) {
        cfg.watchlists.remove(query);
        confy::store("zigfi", cfg).expect("Failed to save.");
        stdout()
            .write("Watchlist has been deleted.".as_bytes())
            .expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        stdout().write("Press q to quit...".as_bytes()).expect("Terminal error.");
        stdout().flush().expect("Terminal error.");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    } else {
        stdout()
            .write("Watchlist does not exist.".as_bytes())
            .expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        stdout().write("Press q to quit...".as_bytes()).expect("Terminal error.");
        stdout().flush().expect("Terminal error.");
        let mut event = read().expect("Terminal error.");
        while event != Event::Key(KeyCode::Char('q').into()) {
            event = read().expect("Terminal error.");
        }
    }
}

pub fn help() {
    let help = vec!["zigarg - List of Commands","","Commands","zigarg show <watchlist name>", "zigarg create <watchlist name> <optional: ticker/s>", "zigarg delete <watchlist name>", "zigarg add <watchlistname> <ticker/s>", "zigarg remove <watchlist> <ticker/s>", "zigarg search <name of asset>", "zigarg help", "zigarg list", "", "Developed by Aldrin Zigmund Cortez Velasco", "", "Press q to quit..."];
    for line in help {
        stdout().write(line.as_bytes()).expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
    }
    stdout().flush().expect("Terminal error.");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

pub fn list(cfg: Config) {
    for watchlist in cfg.watchlists.keys() {
        stdout().write(watchlist.as_bytes()).expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
    }
    stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
    stdout().write("Existing watchlist/s displayed.".as_bytes()).expect("Terminal error.");
    stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
    stdout().write("Press q to quit...".as_bytes()).expect("Terminal error.");
    stdout().flush().expect("Terminal error.");
    let mut event = read().expect("Terminal error.");
    while event != Event::Key(KeyCode::Char('q').into()) {
        event = read().expect("Terminal error.");
    }
}

fn prcnt(n1: f64, n2: f64) -> f64 {
    //(n1 - n2 / (n1 + n2/ 2_f64)) * 100
    let sub = n1 - n2;
    let add = n1 + n2;
    let div = add / 2_f64;
    sub / div * 100_f64
}

fn reduc(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}
