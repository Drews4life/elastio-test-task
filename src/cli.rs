use crate::utils::{get_current_timestamp, transform_raw_date};
use clap::parser::ArgMatches;
use clap::{value_parser, Arg, Command};

pub struct ConfigureCliResult {
    pub provider: String,
}

pub struct GetCliResult {
    pub address: String,
    pub date_timestamp: i64,
}

pub enum CliCommand {
    Configure(ConfigureCliResult),
    Get(GetCliResult),
}

type CliResult = Result<CliCommand, ()>;

fn initiate() -> Command<'static> {
    Command::new("weather")
        .about("Elastio Rust Test Task appplication ❤️")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("configure")
                .about("Configure an environment")
                .arg(
                    Arg::with_name("provider")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(false)
                        .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the weather for an address")
                .args(vec![
                    Arg::with_name("address")
                        .takes_value(true)
                        .required(true)
                        .multiple_values(false)
                        .value_parser(value_parser!(String)),
                    Arg::with_name("date")
                        .long("date")
                        .short('d')
                        .takes_value(true)
                        .help("Date in US format mm-dd-yyyy")
                        .required(false),
                ]),
        )
}

fn build_configure_result(arg_matches: &ArgMatches) -> CliResult {
    let provider = arg_matches.get_one::<String>("provider");

    if provider.is_none() {
        return Err(());
    }

    Ok(CliCommand::Configure(ConfigureCliResult {
        provider: provider.unwrap().clone(),
    }))
}

fn build_get_result(arg_matches: &ArgMatches) -> CliResult {
    let default_timestamp = get_current_timestamp();
    let address = arg_matches.get_one::<String>("address");
    // TODO: Add error handling or fallback to 'now' in case of error?
    let timestamp = arg_matches.get_one::<String>("date").map_or_else(
        || default_timestamp,
        |res| transform_raw_date(res).unwrap_or(default_timestamp),
    );

    Ok(CliCommand::Get(GetCliResult {
        address: address.unwrap().to_owned(),
        date_timestamp: timestamp,
    }))
}

pub fn parse() -> CliResult {
    let matches = initiate().get_matches();

    if let Some(configure_matches) = matches.subcommand_matches("configure") {
        return build_configure_result(configure_matches);
    } else if let Some(get_matches) = matches.subcommand_matches("get") {
        return build_get_result(get_matches);
    }

    Err(())
}
