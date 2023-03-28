use chunk_type::ChunkType;
use clap::Parser;
use cli::{Cli, Commands};
use png::Png;
use std::str::FromStr;

mod chunk;
mod chunk_type;
mod cli;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
  let args = Cli::parse();
  match args.command {
    Commands::Encode {
      input,
      message,
      chunk_type,
    } => {
      encode(input, &message, &chunk_type)
    }
    Commands::Decode { input, chunk_type } => {
      decode(input, &chunk_type)
    }
    Commands::Remove { input, chunk_type } => {
      remove(input, &chunk_type)
    }
    Commands::Print { input } => {
      print(input)
    }
  }
}

/// Encodes a message into a PNG file and saves the result
fn encode(
  input: std::path::PathBuf,
  message: &str,
  chunk_type: &str,
) -> Result<()> {
  let mut png = Png::from_file(&input)?;
  let chunk_type = ChunkType::from_str(chunk_type)?;
  png.encode_message(message, chunk_type)?;
  png.save(input)?;
  Ok(())
}

/// Decodes a message from a PNG file
fn decode(input: std::path::PathBuf, chunk_type: &str) -> Result<()> {
  let png = Png::from_file(&input)?;
  let chunk_type = ChunkType::from_str(chunk_type)?;
  let message = png.decode_message(&chunk_type.to_string())?;
  println!("{}", message);
  Ok(())
}

/// Removes a chunk from a PNG file
fn remove(input: std::path::PathBuf, chunk_type: &str) -> Result<()> {
  let mut png = Png::from_file(&input)?;
  let chunk_type = ChunkType::from_str(chunk_type)?;
  match png.remove_chunk(&chunk_type.to_string()) {
    Some(_) => println!("Chunk removed"),
    None => println!("Chunk not found"),
  }
  png.save(input)?;
  Ok(())
}

/// Prints the chunks of a PNG file
fn print(input: std::path::PathBuf) -> Result<()> {
  let png = Png::from_file(&input)?;
  png.print_chunks();
  Ok(())
}
