use std::str::FromStr;
use chunk_type::ChunkType;
use clap::Parser;
use cli::{Cli, Commands};
use png::Png;

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
      encode(input, &message, &chunk_type)?;
      Ok(())
    }
    Commands::Decode { input, chunk_type } => {
      todo!();
    }
    Commands::Remove { input, chunk_type } => {
      todo!();
    }
    Commands::Print { input } => {
      todo!();
    }
    _ => {
      panic!("Unknown command");
    }
  }
}

// Encodes a message into a PNG file and saves the result
fn encode(input: std::path::PathBuf, message: &str, chunk_type: &str) -> Result<()> {
  let mut png = Png::from_file(&input)?;
  let chunk_type = ChunkType::from_str(chunk_type)?;
  png.encode_message(message, chunk_type)?;
  png.save(input)?;
  Ok(())
}
