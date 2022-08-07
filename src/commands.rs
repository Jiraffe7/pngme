use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let bytes = read_file(&args.file_path)?;
    let mut png = Png::try_from(bytes.as_slice())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.into_bytes());
    png.append_chunk(chunk);

    if let Some(output_path) = args.output_path {
        write_file(&output_path, &png.as_bytes())
    } else {
        // in-place
        write_file(&args.file_path, &png.as_bytes())
    }
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let bytes = read_file(&args.file_path)?;
    let png = Png::try_from(bytes.as_slice())?;
    let chunk = png
        .chunk_by_type(&args.chunk_type)
        .ok_or("no chunk found")?;
    let message = chunk.data_as_string()?;
    println!("{message}");
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let bytes = read_file(&args.file_path)?;
    let mut png = Png::try_from(bytes.as_slice())?;
    png.remove_chunk(&args.chunk_type)?;
    write_file(&args.file_path, &png.as_bytes())?;
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let bytes = read_file(&args.file_path)?;
    let png = Png::try_from(bytes.as_slice())?;
    println!("{png}");
    Ok(())
}

fn read_file(file_path: &PathBuf) -> Result<Vec<u8>> {
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_file(file_path: &PathBuf, data: &[u8]) -> Result<()> {
    let f = File::create(&file_path)?;
    let mut writer = BufWriter::new(f);
    writer.write_all(data)?;
    writer.flush()?;
    Ok(())
}
