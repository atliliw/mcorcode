pub mod args;

use args::CliArgs;
use clap::Parser;

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
