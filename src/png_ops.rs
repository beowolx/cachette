use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use std::str::FromStr;

/// Encodes a message into a PNG file and saves the result
pub fn encode(
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
pub fn decode(input: std::path::PathBuf, chunk_type: &str) -> Result<()> {
  let png = Png::from_file(&input)?;
  let chunk_type = ChunkType::from_str(chunk_type)?;
  let message = png.decode_message(&chunk_type.to_string())?;
  println!("{}", message);
  Ok(())
}

/// Removes a chunk from a PNG file
pub fn remove(input: std::path::PathBuf, chunk_type: &str) -> Result<()> {
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
pub fn print_chunks(input: std::path::PathBuf) -> Result<()> {
  let png = Png::from_file(&input)?;
  png.print_chunks();
  Ok(())
}
