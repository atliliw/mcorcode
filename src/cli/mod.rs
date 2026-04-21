pub mod args;
pub mod repl;

use args::CliArgs;
use clap::Parser;

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

pub use repl::Repl;
