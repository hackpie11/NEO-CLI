# Neo

> "The terminal, but smarter."

Neo is a modern, fast, and user-friendly CLI tool designed to make your terminal experience better. It features simple file management, built-in web search, and a beautiful UI.

## Features

- **File Management**: Create, delete, rename, and move files/folders with ease.
- **Web Search**: Search the web directly from your terminal and open results in your browser.
- **Beautiful UI**: Colored output, spinners, and clear error messages.
- **Cross-Platform**: Works on Linux, macOS, and Windows.
- **Fast**: Built with Rust for speed and stability.

## Installation

### Prerequisites
- Rust and Cargo installed (https://rustup.rs/)

### Build from Source

```bash
git clone https://github.com/hackpie11/neo.git
cd neo
cargo build --release
sudo cp target/release/neo /usr/local/bin/
```

## Usage

### Help
```bash
neo --help
```

### File Management

Create a new folder:
```bash
neo new folder myProject
```

Create a new file:
```bash
neo new file index.html
```

Delete a file or folder:
```bash
neo delete myProject
```

Rename an item:
```bash
neo rename old_name new_name
```

Move an item:
```bash
neo move source_path dest_path
```

### Web Search

Search for something:
```bash
neo search "rust programming language"
```

Search and open the top result immediately:
```bash
neo search "rust programming language" --open
```

### Configuration

Initialize configuration:
```bash
neo config init
```

Show configuration location:
```bash
neo config where
```

## Configuration

Configuration is stored in `~/.config/neo/config.toml` (on Linux).

```toml
[user]
user_name = "User"
theme = "default"
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](LICENSE)
