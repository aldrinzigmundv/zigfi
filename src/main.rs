#![forbid(unsafe_code)]

use zigarg::Arguments;
use zigfi::{self, add, colorswap, delete, display, help, list, new, remove, search, startup};

mod terminal;

fn main() {
    //Makes panic! reset terminal back from Alternate Screen first before crashing for cleaner error message
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        terminal::cleanup();
        default_panic(info);
    }));

    //Captures arguments using zigarg crate
    let arguments = Arguments::new();

    //Sets up default configuration if not available
    startup();

    //Sets up terminal (Alternate Screen)
    terminal::setup();

    //Processes arguments and executes request
    if !arguments.has_args() {
        display("default", "1d");
    } else if arguments.get(1) == Some(&"show".to_string()) {
        let mut interval = "1d";
        if arguments.exist("1mo") {
            interval = "1mo";
        }
        if arguments.exist("1y") {
            interval = "1y";
        }
        display(
            arguments
                .get_value("show")
                .expect("Something wrong with arguments. Please, double check."),
            interval,
        );
    } else if arguments.exist("search") {
        search(
            arguments
                .get_value("search")
                .expect("Something wrong with arguments. Please, double check."),
        );
    } else if arguments.exist("new") {
        new(
            arguments
                .get_value("new")
                .expect("Something wrong with arguments. Please, double check."),
            arguments.get_after_index(3),
        );
    } else if arguments.exist("add") {
        add(
            arguments
                .get_value("add")
                .expect("Something wrong with arguments. Please, double check."),
            arguments.get_after_index(3),
        );
    } else if arguments.exist("remove") {
        remove(
            arguments
                .get_value("remove")
                .expect("Something wrong with arguments. Please, double check."),
            arguments.get_after_index(3),
        );
    } else if arguments.exist("delete") {
        delete(
            arguments
                .get_value("delete")
                .expect("Something wrong with arguments. Please, double check."),
        );
    } else if arguments.exist("help") || arguments.exist("--help") || arguments.exist("-h") {
        help();
    } else if arguments.exist("list") {
        list();
    } else if arguments.exist("colorswap") {
        colorswap();
    } else {
        terminal::write_then_nextline("Command not found.");
        terminal::skip_line();
        help();
    }

    //Resets terminal back from Alternate Screen before Exit
    terminal::cleanup();
}
