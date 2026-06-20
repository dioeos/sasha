mod cli;
mod command;

use command::CargoParser;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = CargoParser::parse_from(cli::cargo_args());
    let _ = cli::send_request(args.request_pattern).await;
}
