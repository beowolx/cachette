#![allow(dead_code)]
use clap::Parser;
use cli::{Cli, Commands};
use png_ops::{decode, encode, print_chunks, remove};

mod chunk;
mod chunk_type;
mod cli;
mod png;
mod png_ops;
mod aes;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
  let args = Cli::parse();
  match args.command {
    Commands::Encode {
      input,
      message,
      chunk_type,
    } => encode(input, &message, &chunk_type),
    Commands::Decode { input, chunk_type } => decode(input, &chunk_type),
    Commands::Remove { input, chunk_type } => remove(input, &chunk_type),
    Commands::Print { input } => print_chunks(input),
  }
}
