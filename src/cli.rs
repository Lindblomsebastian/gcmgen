use clap::{command, Arg, ArgMatches};

pub fn build_cli() -> ArgMatches {
    command!()
        .version("1.0")
        .author("Sebastian Stan")
        .about("Generates commit messages using AI")
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Specify the OpenAI model to use")
        )
        .arg(
            Arg::new("init")
                .long("init")
                .value_name("TOKEN")
                .help("Initialize the API token for OpenAI")
        )
        .get_matches()
}
