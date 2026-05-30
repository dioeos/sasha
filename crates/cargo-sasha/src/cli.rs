use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(bin_name = "cargo sasha")]
pub struct SashaCargoParser {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Update {
        #[command(subcommand)]
        target: UpdateTarget
    },
    Start {
        #[command(subcommand)]
        target: StartTarget
    },
    Stop {
        #[command(subcommand)]
        target: StopTarget
    },
    Logs {
        #[command(subcommand)]
        target: LogsTarget
    },
    MarkWindow {
        slot: u8
    },
    FocusWindow {
        slot: u8
    }
}

#[derive(Subcommand)]
pub enum UpdateTarget {
    Daemon,
    Service,
}

#[derive(Subcommand)]
pub enum StopTarget {
    Service
}
#[derive(Subcommand)]
pub enum StartTarget {
    Service
}
#[derive(Subcommand)]
pub enum LogsTarget {
    Service
}

pub fn cargo_args() -> Vec<String> {
    let mut raw_args: Vec<String> = std::env::args().collect();
    
    if raw_args.get(1).map(String::as_str) == Some("sasha") {
        raw_args.remove(1);
    }
    raw_args
}

pub fn determine_script_for(cmd: Command) -> &'static str {
    match cmd {
        Command::Update { target: UpdateTarget::Daemon } => {
            "./scripts/daemon/update_daemon.sh"
        }
        Command::Update { target: UpdateTarget::Service } => {
            "./scripts/service/update_sasha_service.sh"
        }
        Command::Start { target: StartTarget::Service } => {
            "./scripts/service/start_sasha_service.sh"
        }
        Command::Stop { target: StopTarget::Service } => {
            "./scripts/service/stop_sasha_service.sh"
        }
        Command::Logs { target: LogsTarget::Service } => {
            "./scripts/utils/start_live_sasha_logs.sh"
        }
        Command::MarkWindow { slot } => {
            "./scripts/test.sh"
        }
        Command::FocusWindow { slot } => {
            "./scripts/test.sh"
        }
    }
}
