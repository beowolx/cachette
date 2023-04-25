# Cachette

Cachette (hideout in french) is a Rust-based command-line interface (CLI) program that enables users to securely and efficiently conceal secret messages in PNG image files using [steganography](https://en.wikipedia.org/wiki/Steganography). Its simple and intuitive interface ensures that confidential data can be hidden within image files without altering the image's visual appearance.

## Features

- Rust-based, lightweight, and efficient.
- Conceals messages within PNG image files using steganography.
- Encrypts secret messages using [AES-256](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) encryption.
- User needs to provide a strong password (min. 18 characters long) to encrypt the message.
- Preserves the visual appearance of the original image.
- Simple and intuitive command-line interface.

## Installation

To install Cachette, ensure that you have Rust and Cargo installed on your system. If not, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

Clone the repository:

```bash
https://github.com/LuisCardosoOliveira/cachette.git
```

Navigate to the project directory and build the project:

```bash
cd cachette
cargo build --release
```

The binary file will be available in the `target/release` folder.

You need to go to this folder and run the binary from there:

```bash
cd target/release
./cachette --help
```

## Usage

Cachette uses _chunk types_ to encode the message in the PNG file. They are 
basically 4 alphabetic characters that follows the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html).

To use Cachette, simply run the binary file with the appropriate commands and arguments:

- To encode a message within an image:

```bash
./cachette encode <PNG_FILE_PATH> <CHUNK_TYPE> <MESSAGE>

# Example:
./cachette encode ./cat.png teXt "This is a secret message!"
```

You will then be prompted to provide a password. This password will be used to
decrypt your message and should be **at least 18 characters long.**

- To decode a hidden message from an image:

```bash
./cachette decode ./cat.png teXt
```

- To remove a hidden message from an image:

```bash
./cachette remove ./cat.png teXt
```

- To print the original image without altering it:

```bash
./cachette print ./cat.png
```

- For additional options and help:

```bash
./cachette --help
```

## Encryption and Hashing: AES-256 and Argon2

Cachette utilizes both AES-256 encryption and Argon2 hashing to ensure the security of your secret messages within the PNG image files. This section provides an overview of these cryptographic techniques and how they are used in the program.

### AES-256 Encryption

[Advanced Encryption Standard (AES)](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) is a symmetric encryption algorithm widely used for securing sensitive data. AES-256, in particular, employs a key size of 256 bits, offering a high level of security. In Cachette, the secret message is encrypted using AES-256 before being concealed within the image file using steganography.

When encoding a message, the program prompts the user to provide a password. This password is then used as a key to encrypt the secret message with AES-256. The encrypted message is embedded in the PNG file, ensuring that only someone with the correct password can decode and access the message.

### Argon2 Hashing

[Argon2](https://en.wikipedia.org/wiki/Argon2) is a modern and secure key derivation function designed to hash passwords. It has been selected as the winner of the [Password Hashing Competition](https://password-hashing.net/) and is recommended for various security applications. Cachette uses Argon2 to hash the user-provided password for added security.

When the user provides a password for encoding or decoding a message, the password is hashed using Argon2 with a fixed salting value. The hashed password is then used as a key for AES-256 encryption or decryption. This method ensures that even if an attacker manages to access the hashed password, they would still need to perform a computationally expensive brute-force attack to retrieve the original password.

However, it is essential to note that using a fixed salting value can have security implications, as mentioned in the Disclaimer section. To mitigate these risks, users should employ strong, unique passwords.

By combining AES-256 encryption and Argon2 hashing, Cachette aims to provide a robust and secure method for concealing secret messages within PNG image files.

## Disclaimer
Although Cachette encrypts secret messages using AES-256 and hashes passwords with Argon2, there are always risks associated with data security. In particular, Cachette uses a fixed salting value for hashing passwords, which can make it easier for attackers to perform precomputed attacks or rainbow table attacks. This design choice was made to simplify password verification but could have security implications. Users should be aware of these risks and employ strong, unique passwords to minimize potential vulnerabilities.

## Contributing
I welcome contributions from the community! If you'd like to contribute to Cachette, please do it.
