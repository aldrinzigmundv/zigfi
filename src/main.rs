use confy;
use tokio::runtime::Runtime;
use zigarg::Arguments;
use zigfi::{self, add, create, delete, display, remove, search, start_ui, startup, help, list, Config};

use crossterm::{cursor::{Show, MoveToNextLine}, terminal, ExecutableCommand};
use std::{io::{stdout, Write}};

use yahoo_finance_api as yahoo;

fn main() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        terminal::disable_raw_mode().unwrap();
        stdout().execute(terminal::LeaveAlternateScreen).unwrap();
        stdout().execute(Show).unwrap();
        default_panic(info);
    }));
    let arguments = Arguments::new();
    let rt = Runtime::new().expect("Failed to start Runtime");
    let yahoo = yahoo::YahooConnector::new();
    startup();
    let cfg: Config = confy::load("zigfi").expect("Failed to load zigfi configuration.");
    start_ui().expect("Failed to start terminal.");
    if !arguments.has_args() {
        display(rt, yahoo, cfg, "default", "1d");
    } else if arguments.get(1) == Some(&"show".to_string()) {
        let mut interval = "1d";
        if arguments.exist("1wk") {
            interval = "1wk";
        }
        if arguments.exist("1mo") {
            interval = "1mo";
        }
        display(rt, yahoo, cfg, arguments.get(2).expect("Something wrong with arguments. Please, double check."), interval);
    } else if arguments.exist("search") {
        search(rt, yahoo, arguments.get(2).expect("Something wrong with arguments. Please, double check."));
    } else if arguments.exist("create") {
        create(
            rt,
            yahoo,
            cfg,
            arguments.get(2).expect("Something wrong with arguments. Please, double check."),
            arguments.0[3..].iter().collect::<Vec<_>>(),
        );
    } else if arguments.exist("add") {
        add(
            rt,
            yahoo,
            cfg,
            arguments.get(2).expect("Something wrong with arguments. Please, double check."),
            arguments.0[3..].iter().collect::<Vec<_>>(),
        );
    } else if arguments.exist("remove") {
        remove(
            cfg,
            arguments.get(2).expect("Something wrong with arguments. Please, double check."),
            arguments.0[3..].iter().collect::<Vec<_>>(),
        );
    } else if arguments.exist("delete") {
        delete(cfg, arguments.get(2).expect("Something wrong with arguments. Please, double check."));
    } else if arguments.exist("help") || arguments.exist("--help") || arguments.exist("-h") {
        help();
    } else if arguments.exist("list") {
        list(cfg);
    } else {
        stdout().write("Command not found.".as_bytes()).expect("Terminal error.");
        stdout().execute(MoveToNextLine(1)).expect("Terminal error.");
        help();
    }
    terminal::disable_raw_mode().expect("Terminal error.");
    stdout().execute(terminal::LeaveAlternateScreen).expect("Terminal error.");
    stdout().execute(Show).expect("Terminal error.");
}
