use crate::chunk_type::ChunkType;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cahette")]
#[command(bin_name = "cachette")]
#[command(about = "A program to hide secret messages on PNG files", long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  /// Encode a message in a PNG file
  Encode {
    /// The PNG file to encode
    input: std::path::PathBuf,

    /// The chunk type to use
    chunk_type: String,

    /// The message to encode
    message: String,
  },
  /// Decode a message in a PNG file
  Decode {
    /// The PNG file to decode
    input: std::path::PathBuf,

    /// The chunk type to use
    #[clap(short, long, default_value = "tEXt")]
    chunk_type: ChunkType,
  },
  /// Remove a message from a PNG file
  Remove {
    /// The PNG file to remove the message from
    input: std::path::PathBuf,

    /// The chunk type to use
    #[clap(short, long, default_value = "tEXt")]
    chunk_type: ChunkType,
  },
  /// Print a message from a PNG file
  Print {
    /// The PNG file to print the message from
    input: std::path::PathBuf,
  },
}
