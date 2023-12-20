mod operations;

extern crate clap;
use clap::{Parser, Subcommand};
use std::error::Error;

type CliResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// scaffold a new day
    Scaffold {
        /// day to scaffold
        day: u8,
    },

    /// download input for a day
    Download {
        /// day to download
        day: u8,
    },

    /// scaffold and download input for a day
    Create {
        /// day to scaffold and download
        day: u8,
    },

    /// run a day's solution
    Run {
        /// day's solution to run
        day: String,
    },

    /// run tests for a day's solution
    Tests {
        /// run the tests for the provided day
        day: String,
    },
}

#[derive(Debug)]
pub struct Config {
    command: Option<Commands>,
}

pub fn get_args() -> CliResult<Config> {
    let cli = Cli::parse();
    Ok(Config {
        command: cli.command,
    })
}

pub fn run(config: Config) -> CliResult<()> {
    match config.command {
        Some(Commands::Scaffold { day }) => {
            let result = operations::scaffold(day);
            match result {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("scaffold command - {}", e).into());
                }
            }
        }
        Some(Commands::Download { day }) => {
            let result = operations::download(day);
            match result {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("download command - {}", e).into());
                }
            }
        }
        Some(Commands::Create { day }) => {
            let result = operations::scaffold(day);
            match result {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("scaffold command - {}", e).into());
                }
            }
            let result = operations::download(day);
            match result {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("download command - {}", e).into());
                }
            }
        }
        Some(Commands::Run { day }) => {
            println!("Running day {}", day);
            let result = operations::run(day);
            match result {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("run command - {}", e).into());
                }
            }
        }
        Some(Commands::Tests { day }) => {
            let result = operations::test(day);
            match result {
                Ok(_) => (),
                Err(e) => {
                    return Err(format!("test command - {}", e).into());
                }
            }
        }
        None => {}
    }
    Ok(())
}
