#![forbid(unsafe_code)]

use atty::Stream;
use zigarg::Arguments;
use zigfi::{
    self, add, colorswap, delete, display, help, list, new, print, print_json, remove, search,
    startup,
};

mod output;

fn main() {
    ///Makes panic! reset output back from Alternate Screen first before crashing for cleaner error message
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        output::cleanup();
        default_panic(info);
    }));

    ///Captures arguments using zigarg crate
    let arguments = Arguments::new();

    ///Sets up default configuration if not available
    startup();

    ///Variable to suppress cleanup on output if requester is not tty preventing ANSI escapes from being piped
    let mut clean_up_required = true;

    //Processes arguments and executes request
    if !arguments.has_args() && atty::is(Stream::Stdout) {
        display("default", "1d");
    } else if !arguments.has_args() && !arguments.exist("--json") {
        print("default", "1d");
    } else if !arguments.has_args() {
        print_json("default", "1d");
    } else if arguments.exist("show") && atty::is(Stream::Stdout) {
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
    } else if arguments.exist("show") && arguments.exist("--json") {
        clean_up_required = false;
        let mut interval = "1d";
        if arguments.exist("1mo") {
            interval = "1mo";
        }
        if arguments.exist("1y") {
            interval = "1y";
        }
        print_json(
            arguments
                .get_value("show")
                .expect("Something wrong with arguments. Please, double check."),
            interval,
        );
    } else if arguments.exist("show") {
        clean_up_required = false;
        let mut interval = "1d";
        if arguments.exist("1mo") {
            interval = "1mo";
        }
        if arguments.exist("1y") {
            interval = "1y";
        }
        print(
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
        output::write_then_nextline("Command not found.");
        output::skip_line();
        help();
    }

    //Resets output back from Alternate Screen before Exit
    if clean_up_required {
        output::cleanup();
    }
}
