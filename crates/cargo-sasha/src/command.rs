use clap::{Parser, Subcommand};
use serde::{Serialize};


#[derive(Parser)]
#[command(bin_name = "cargo sasha")]
pub struct CargoParser {
    #[command(subcommand)]
    pub request_pattern: RequestPattern,
}


#[derive(Subcommand, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RequestPattern {
    MarkWindow { slot: u8 },
    FocusWindow { slot: u8 }
}
