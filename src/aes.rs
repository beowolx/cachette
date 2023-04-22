use aes_gcm::aead::{
  generic_array::{
    typenum::{U12, U32},
    GenericArray,
  },
  Aead, KeyInit, OsRng,
};
use aes_gcm::{Aes256Gcm, Nonce};

use rand::Rng;

pub fn encrypt_message(
  message: &str,
) -> (Vec<u8>, GenericArray<u8, U32>, GenericArray<u8, U12>) {
  // Generate a random 256-bit key
  let key = Aes256Gcm::generate_key(&mut OsRng);

  // Generate a random 96-bit nonce
  let nonce_rnd = rand::thread_rng().gen::<[u8; 12]>();
  let nonce = Nonce::from_slice(&nonce_rnd);

  // Create a new AES-GCM cipher with the generated key
  let cipher = Aes256Gcm::new(&key);

  // Encrypt the message using the cipher and the nonce

  let ciphertext = cipher
    .encrypt(nonce, message.as_bytes())
    .expect("encryption failure!");

  (ciphertext, key, *nonce)
}

pub fn decrypt_message(
  ciphertext: &[u8],
  key: &GenericArray<u8, U32>,
  nonce: &GenericArray<u8, U12>,
) -> String {
  // Create a new AES-GCM cipher with the generated key
  let cipher = Aes256Gcm::new(key);

  // Decrypt the message using the cipher and the nonce
  let plaintext = cipher
    .decrypt(nonce, ciphertext)
    .expect("decryption failure!");

  // Convert the decrypted bytes to a String and return it
  String::from_utf8(plaintext).expect("invalid UTF-8")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_encrypt_decrypt() {
    let message = "This is a secret message!";

    // Encrypt the message
    let (ciphertext, key, nonce) = encrypt_message(message);

    // Decrypt the message
    let decrypted_message = decrypt_message(&ciphertext, &key, &nonce);

    // Assert that the decrypted message is equal to the original message
    assert_eq!(message, decrypted_message);
  }

  #[test]
  fn test_decrypt_with_wrong_key() {
    let message = "This is a secret message!";

    // Encrypt the message
    let (ciphertext, mut key, nonce) = encrypt_message(message);

    // Tamper with the key
    key[0] ^= 0x01;

    // Attempt to decrypt the message with the wrong key
    let result =
      std::panic::catch_unwind(|| decrypt_message(&ciphertext, &key, &nonce));

    // Assert that the decryption failed
    assert!(result.is_err());
  }

  #[test]
  fn test_decrypt_with_wrong_nonce() {
    let message = "This is a secret message!";

    // Encrypt the message
    let (ciphertext, key, mut nonce) = encrypt_message(message);

    // Tamper with the nonce
    nonce[0] ^= 0x01;

    // Attempt to decrypt the message with the wrong nonce
    let result =
      std::panic::catch_unwind(|| decrypt_message(&ciphertext, &key, &nonce));

    // Assert that the decryption failed
    assert!(result.is_err());
  }
}
