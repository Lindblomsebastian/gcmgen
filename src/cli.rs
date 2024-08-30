use clap::{command, Arg, ArgAction, ArgMatches};

pub fn build_cli() -> ArgMatches {
    command!()
        .version("1.0")
        .author("Sebastian Stan")
        .about("Generates commit messages using AI")
        .arg(
            Arg::new("init")
                .long("init")
                .action(ArgAction::SetTrue)
                .help("Initialize the config"),
        )
        .get_matches()
}
