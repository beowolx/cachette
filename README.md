# Cachette

Cachette (hideout in french) is a Rust-based command-line interface (CLI) program that enables users to securely and efficiently conceal secret messages in PNG image files using [steganography](https://en.wikipedia.org/wiki/Steganography). Its simple and intuitive interface ensures that confidential data can be hidden within image files without altering the image's visual appearance.

## Features

- Rust-based, lightweight, and efficient.
- Conceals messages within PNG image files using steganography.
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

## Contributing
I welcome contributions from the community! If you'd like to contribute to Cachette, please do it.
