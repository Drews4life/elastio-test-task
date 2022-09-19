use crate::cli::CliCommand;
use crate::configuration::{get_entry, set_entry};
use crate::weather::provider_factory::Producer;

mod cli;
mod configuration;
mod utils;
mod weather;

fn main() {
    match cli::parse() {
        Ok(cli_result) => match cli_result {
            CliCommand::Configure(configure_result) => {
                set_entry("saved_provider", &configure_result.provider);
            }
            CliCommand::Get(get_result) => {
                let address = get_result.address;

                if let Some(provider) = get_entry("saved_provider") {
                    let provider = Producer::create(&provider);

                    let forecast = provider.get(&address, get_result.date_timestamp);

                    println!("{forecast} in {address}");
                } else {
                    eprint!("Call 'test-task configure <provider>' command before calling 'get'!");
                };
            }
        },
        Err(_) => {
            eprint!("Something went wrong");
        }
    }
}
