use clap::Parser;
use cli::{Cli, Commands};

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
      todo!();
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
