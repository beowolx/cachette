use crate::aes::encrypt_message;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use base64::{engine::general_purpose, Engine as _};
use std::str::FromStr;

fn get_password() -> Result<String> {
  match rpassword::prompt_password("Your password: ") {
    Ok(password) => {
      if password.len() < 18 {
        Err("Password must be at least 18 characters long".into())
      } else {
        Ok(password)
      }
    }
    Err(_) => Err("Failed to read password".into()),
  }
}

pub fn encode(
  input: std::path::PathBuf,
  message: &str,
  chunk_type: &str,
) -> Result<()> {
  let password = get_password()?;
  let (encrypted_message, nonce) = encrypt_message(message, &password);
  let mut png = Png::from_file(&input)?;
  let chunk_type = ChunkType::from_str(chunk_type)?;
  let nonce_chunk_type_str = format!("n{}", &chunk_type.to_string()[1..]);
  let nonce_chunk_type = ChunkType::from_str(&nonce_chunk_type_str)?;

  let base64_nonce = general_purpose::STANDARD_NO_PAD.encode(&nonce);
  png.encode_message(encrypted_message, chunk_type)?;
  png.encode_message(base64_nonce.into_bytes(), nonce_chunk_type)?;

  png.save(input)?;
  Ok(())
}

/// Decodes a message from a PNG file
pub fn decode(input: std::path::PathBuf, chunk_type: &str) -> Result<()> {
  let password = get_password()?;
  let png = Png::from_file(&input)?;
  let chunk_type = ChunkType::from_str(chunk_type)?;
  let message = png.decode_message(&chunk_type.to_string(), &password)?;
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
