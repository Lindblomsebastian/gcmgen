use clap::{command, crate_version, Arg, ArgAction, ArgMatches};

pub fn build_cli() -> ArgMatches {
    command!()
        .version(crate_version!())
        .author("Sebastian Stan")
        .about("Generates commit messages using AI")
        .arg(
            Arg::new("init")
                .long("init")
                .action(ArgAction::SetTrue)
                .help("Initialize the config"),
        )
        .arg(
            Arg::new("set-default")
                .long("set-default")
                .help("Sets the default config. Example: gcmgen --set-default OpenAI"),
        )
        .arg(
            Arg::new("prefix")
                .long("prefix")
                .short('p')
                .help("Set prefix for commit message. Example: gcmgen -p TICKET-123"),
        )
        .arg(
            Arg::new("pull-request")
                .long("pr")
                .help("Opens up a new PR in the browser with generated description and title")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list-services")
                .long("ls")
                .help("Lists all configured services")
                .action(ArgAction::SetTrue),
        )
        .get_matches()
}
