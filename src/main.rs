use clap::Parser;

use crate::args::Commands;
use crate::args::Commands::{Decode, Encode, Print, Remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Commands::parse();

    match args {
        Encode(args) => commands::encode(args),
        Decode(args) => commands::decode(args),
        Remove(args) => commands::remove(args),
        Print(args) => commands::print_chunks(args),
    }
}
