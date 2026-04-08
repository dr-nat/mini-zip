# mini_zip 🗜️

A lightweight command-line file compression tool built in Rust.

## Overview

`mini_zip` takes a file as input and compresses it into a `.zip` archive, saving the output in the same directory as the original file.

## Features

- Accepts a file path as a command-line argument
- Validates that the input is a real, existing file
- Compresses the file using the Deflate compression method
- Saves the output `.zip` file in the same directory as the original
- Returns a success message on completion

## Project Structure

```
mini_zip/
├── src/
│   ├── main.rs          # Entry point, handles output and error display
│   ├── inputs.rs        # CLI argument parsing and file validation
│   ├── path_to_file.rs  # Opens file and returns BufReader + file name
│   └── compress.rs      # Core compression logic using the zip crate
├── Cargo.toml
└── README.md
```

## Dependencies

```toml
[dependencies]
zip = "2"
```

## Usage

```bash
cargo run -- /path/to/your/file.txt
```

**Example:**

```bash
cargo run -- ~/Documents/Resume.pdf
```

This will create `Resume.pdf.zip` in `~/Documents/`.

## How It Works

1. The CLI module accepts a file path argument and validates it
2. The file reader module opens the file and wraps it in a `BufReader`
3. The compression module:
   - Extracts the file name and parent directory from the path
   - Creates a new `.zip` file in the same directory
   - Wraps it in a `ZipWriter`
   - Sets compression options (Deflate method, Unix permissions)
   - Writes the file contents into the zip
   - Finalizes and closes the zip

## Compression Method

This tool uses the **Deflate** compression algorithm — the standard method used in zip files, balancing speed and compression ratio.

## Error Handling

The tool returns descriptive error messages for:
- Missing or invalid file path arguments
- Files that do not exist
- Failure to read or compress the file

## Planned Features

- [ ] Support for compressing multiple files into one zip
- [ ] Custom output path option
- [ ] Display file size before and after compression
- [ ] Directory compression support

## License

MIT
