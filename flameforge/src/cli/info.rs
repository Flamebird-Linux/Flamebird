// src/cli/info.rs

use clap::Command

pub fn command() -> Command {
    Command::new("info")
        .about("Query packages")
        .long_about("List detailed package information from all available sources")

}
