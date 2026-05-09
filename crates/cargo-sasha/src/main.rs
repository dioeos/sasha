mod cli;
mod script;

use cli::SashaCargoParser;
use clap::Parser;

fn main() {
    let args = SashaCargoParser::parse_from(cli::cargo_args());
    let script = cli::determine_script_for(args.command);
    script::run_script(script);
}
