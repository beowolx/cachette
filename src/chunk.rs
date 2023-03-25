use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

pub const HDLC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug, Clone)]
pub struct Chunk {
  length: u32,
  chunk_type: ChunkType,
  data: Vec<u8>,
  crc: u32,
}

impl Chunk {
  pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
    let length = data.len() as u32;

    // Avoid the allocation of an intermediate Vec<u8> when calculating the crc checksum
    let mut buffer = Vec::with_capacity(chunk_type.bytes().len() + data.len());
    buffer.extend_from_slice(&chunk_type.bytes());
    buffer.extend_from_slice(&data);
    let crc = HDLC.checksum(&buffer);

    Chunk {
      length,
      chunk_type,
      data,
      crc,
    }
  }

  /// The length of the data portion of this chunk
  pub fn length(&self) -> u32 {
    self.length
  }

  /// The `ChunkType` of this chunk
  pub fn chunk_type(&self) -> &ChunkType {
    &self.chunk_type
  }

  /// The raw data contained in this chunk in bytes
  pub fn data(&self) -> &[u8] {
    self.data.as_slice()
  }

  /// Returns the data stored in this chunk as a `String`. This function will return an error
  /// if the stored data is not valid UTF-8.
  pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(self.data.clone())
  }

  /// The CRC of this chunk
  pub fn crc(&self) -> u32 {
    self.crc
  }

  /// Returns this chunk as a byte sequences described by the PNG spec.
  /// The following data is included in this byte sequence in order:
  /// 1. Length of the data *(4 bytes)*
  /// 2. Chunk type *(4 bytes)*
  /// 3. The data itself *(`length` bytes)*
  /// 4. The CRC of the chunk type and data *(4 bytes)*
  pub fn as_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(12 + self.length as usize);

    bytes.extend(&self.length.to_be_bytes());
    bytes.extend(&self.chunk_type.bytes());
    bytes.extend(&self.data);
    bytes.extend(&self.crc.to_be_bytes());

    bytes
  }
}

impl TryFrom<&[u8]> for Chunk {
  type Error = &'static str;

  fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
    let length = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let chunk_type =
      ChunkType::try_from([bytes[4], bytes[5], bytes[6], bytes[7]])?;
    let data = bytes[8..(8 + length as usize)].to_vec();
    let crc = u32::from_be_bytes([
      bytes[8 + length as usize],
      bytes[9 + length as usize],
      bytes[10 + length as usize],
      bytes[11 + length as usize],
    ]);

    let expected_crc = HDLC.checksum(
      chunk_type
        .bytes()
        .iter()
        .chain(data.iter())
        .copied()
        .collect::<Vec<u8>>()
        .as_slice(),
    );

    if crc != expected_crc {
      return Err("Invalid CRC");
    }

    Ok(Chunk {
      length,
      chunk_type,
      data,
      crc,
    })
  }
}

impl std::fmt::Display for Chunk {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.chunk_type)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::chunk_type::ChunkType;
  use std::str::FromStr;

  fn testing_chunk() -> Chunk {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
      .to_be_bytes()
      .iter()
      .chain(chunk_type.iter())
      .chain(message_bytes.iter())
      .chain(crc.to_be_bytes().iter())
      .copied()
      .collect();

    Chunk::try_from(chunk_data.as_ref()).unwrap()
  }

  #[test]
  fn test_new_chunk() {
    let chunk_type = ChunkType::from_str("RuSt").unwrap();
    let data = "This is where your secret message will be!"
      .as_bytes()
      .to_vec();
    let chunk = Chunk::new(chunk_type, data);
    assert_eq!(chunk.length(), 42);
    assert_eq!(chunk.crc(), 2882656334);
  }

  #[test]
  fn test_chunk_length() {
    let chunk = testing_chunk();
    assert_eq!(chunk.length(), 42);
  }

  #[test]
  fn test_chunk_type() {
    let chunk = testing_chunk();
    assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
  }

  #[test]
  fn test_chunk_string() {
    let chunk = testing_chunk();
    let chunk_string = chunk.data_as_string().unwrap();
    let expected_chunk_string =
      String::from("This is where your secret message will be!");
    assert_eq!(chunk_string, expected_chunk_string);
  }

  #[test]
  fn test_chunk_crc() {
    let chunk = testing_chunk();
    assert_eq!(chunk.crc(), 2882656334);
  }

  #[test]
  fn test_valid_chunk_from_bytes() {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
      .to_be_bytes()
      .iter()
      .chain(chunk_type.iter())
      .chain(message_bytes.iter())
      .chain(crc.to_be_bytes().iter())
      .copied()
      .collect();

    let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

    let chunk_string = chunk.data_as_string().unwrap();
    let expected_chunk_string =
      String::from("This is where your secret message will be!");

    assert_eq!(chunk.length(), 42);
    assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    assert_eq!(chunk_string, expected_chunk_string);
    assert_eq!(chunk.crc(), 2882656334);
  }

  #[test]
  fn test_invalid_chunk_from_bytes() {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656333;

    let chunk_data: Vec<u8> = data_length
      .to_be_bytes()
      .iter()
      .chain(chunk_type.iter())
      .chain(message_bytes.iter())
      .chain(crc.to_be_bytes().iter())
      .copied()
      .collect();

    let chunk = Chunk::try_from(chunk_data.as_ref());

    assert!(chunk.is_err());
  }

  #[test]
  pub fn test_chunk_trait_impls() {
    let data_length: u32 = 42;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "This is where your secret message will be!".as_bytes();
    let crc: u32 = 2882656334;

    let chunk_data: Vec<u8> = data_length
      .to_be_bytes()
      .iter()
      .chain(chunk_type.iter())
      .chain(message_bytes.iter())
      .chain(crc.to_be_bytes().iter())
      .copied()
      .collect();

    let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

    let _chunk_string = format!("{}", chunk);
  }
}
