use clap::App;
use clap::Arg;

pub struct CliArguments {
    pub searched_directory: String,
    pub recursive_search: bool,
    pub only_summary: bool,
}

pub fn run_cli() -> CliArguments {
    let matches = App::new("fbfc")
        .version("0.1.0")
        .author("Krzysztof Hrynczenko")
        .about(concat!(
            "Finds banned function calls in C/C++ source files according",
            "to the \"Security Development Lifecycle (SDL) Banned Function Calls\" article."
        ))
        .arg(
            Arg::with_name("DIRECTORY")
                .required(true)
                .help("search directory"),
        )
        .arg(
            Arg::with_name("r")
                .short("r")
                .required(false)
                .help("search recursively within directory"),
        )
        .arg(
            Arg::with_name("only-summary")
                .long("only-summary")
                .required(false)
                .help("output only a summary"),
        )
        .get_matches();

    let recursive_search = matches.is_present("r");
    let searched_directory = matches.value_of("DIRECTORY").unwrap().to_string(); // DIRECTORY is requried so unwrap cannot fail (unless arguments is not UTF8)
    let only_summary = matches.is_present("only-summary");
    return CliArguments {
        searched_directory,
        recursive_search,
        only_summary,
    };
}
