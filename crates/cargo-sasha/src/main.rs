use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Subcommand)]
enum Commands {
    Update {
        #[arg(long)]
        daemon: bool,

        #[arg(long)]
        service: bool,
    },

    Stop {
        #[arg(long)]
        service: bool
    }
}

#[derive(Parser)]
#[command(bin_name = "cargo sasha")]
struct SashaCargoParser {
    #[command(subcommand)]
    command: Commands,
}

fn run_script(path: &str) {
    let status = Command::new(path)
        .status()
        .expect("Failed to run script");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn main() {
    let mut raw_args: Vec<String> = std::env::args().collect();

    if raw_args.get(1).map(String::as_str) == Some("sasha") {
        raw_args.remove(1);
    }

    let args = SashaCargoParser::parse_from(raw_args);

    match args.command {
        Commands::Update { daemon, service } => {
            if daemon {
                run_script("./scripts/update_daemon.sh");
            }
            if service {
                run_script("./scripts/update_service.sh");
            }
        }

        Commands::Stop { service } => {
            if service {
                run_script("./scripts/stop_sasha_service.sh");
            }
        }
    }
}
