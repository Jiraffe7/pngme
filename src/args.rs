use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Parser)]
/// Encodes a message into a PNG file and saves the result
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
    #[clap(value_parser)]
    pub message: String,
    #[clap(value_parser)]
    pub output_path: Option<PathBuf>,
}

#[derive(Debug, Parser)]
/// Searches for a message hidden in a PNG file and prints the message if one is found
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
/// Removes a chunk from a PNG file and saves the result
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Debug, Parser)]
/// Prints all of the chunks in a PNG file
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file_path: PathBuf,
}
