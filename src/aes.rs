use crate::Result;
use aes_gcm::aead::{
  generic_array::{typenum::U12, GenericArray},
  Aead, KeyInit,
};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::PasswordHasher;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, Version};

use rand::Rng;

pub fn encrypt_message(
  message: &str,
  password: &str,
) -> (Vec<u8>, GenericArray<u8, U12>) {
  let hash = compute_password_hash(password).expect("Failed to hash password");
  let key = &hash.as_bytes()[32..64];

  // Generate a random 96-bit nonce
  let nonce_rnd = rand::thread_rng().gen::<[u8; 12]>();
  let nonce = Nonce::from_slice(&nonce_rnd);

  // Create a new AES-GCM cipher with the key
  let cipher = Aes256Gcm::new(key.into());

  // Encrypt the message using the cipher and the nonce
  let encrypted_text = cipher
    .encrypt(nonce, message.as_bytes())
    .expect("encryption failure!");

  (encrypted_text, *nonce)
}

pub fn decrypt_message(
  ciphertext: &[u8],
  password: &str,
  nonce: &GenericArray<u8, U12>,
) -> String {
  let hash = compute_password_hash(password).expect("Failed to hash password");
  let key = &hash.as_bytes()[32..64];

  let cipher = Aes256Gcm::new(key.into());

  // Decrypt the message using the cipher and the nonce
  let decrypted_message = cipher
    .decrypt(nonce, ciphertext)
    .expect("decryption failure!");

  // Convert the decrypted bytes to a String and return it
  String::from_utf8(decrypted_message).expect("invalid UTF-8")
}

fn compute_password_hash(password: &str) -> Result<String> {
  let fixed_salt = SaltString::from_b64("quVHYU6WoFQFvRPy0PLuhQ")
    .expect("Failed to parse salt");
  let password_hash = Argon2::new(
    Algorithm::Argon2id,
    Version::V0x13,
    Params::new(15000, 2, 1, None).expect("Failed to create params"),
  )
  .hash_password(password.as_bytes(), &fixed_salt)
  .expect("Failed to hash password")
  .to_string();

  Ok(password_hash)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_encrypt_decrypt() {
    let message = "This is a secret message!";
    let password = "bXR3be-ujj!hP@NDaTUnnBuBQ";

    // Encrypt the message
    let (ciphertext, nonce) = encrypt_message(message, password);

    // Decrypt the message
    let decrypted_message = decrypt_message(&ciphertext, &password, &nonce);

    // Assert that the decrypted message is equal to the original message
    assert_eq!(message, decrypted_message);
  }

  #[test]
  fn test_decrypt_with_wrong_key() {
    let message = "This is a secret message!";
    let password = "bXR3be-ujj!hP@NDaTUnnBuBQ";

    // Encrypt the message
    let (ciphertext, nonce) = encrypt_message(message, password);

    // Attempt to decrypt the message with the wrong key
    let wrong_password = "wrong password garbasish";
    let result = std::panic::catch_unwind(|| {
      decrypt_message(&ciphertext, &wrong_password, &nonce)
    });

    // Assert that the decryption failed
    assert!(result.is_err());
  }

  #[test]
  fn test_decrypt_with_wrong_nonce() {
    let message = "This is a secret message!";
    let password = "bXR3be-ujj!hP@NDaTUnnBuBQ";

    // Encrypt the message
    let (ciphertext, mut nonce) = encrypt_message(message, password);

    // Tamper with the nonce
    nonce[0] ^= 0x01;

    // Attempt to decrypt the message with the wrong nonce
    let result = std::panic::catch_unwind(|| {
      decrypt_message(&ciphertext, &password, &nonce)
    });

    // Assert that the decryption failed
    assert!(result.is_err());
  }
}
