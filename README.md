# nlf

[![Crates.io Version](https://img.shields.io/crates/v/nlf?style=for-the-badge&color=blue)](https://crates.io/crates/nlf)

English | [简体中文](README.ZH-CN.md)

Append a newline character (LF) to the end of the file.

## Installation

### Cargo

```sh
cargo install nlf --locked --profile release-small
```

## Usage

```sh
# Append a newline character (LF) to the end of the file if it does not exist
nlf a.txt

# Fix newline characters of all .txt files in the dir directory (append if there is no newline at the end)
find dir -type f -name '*.txt' -exec nlf {} \;
```

## License

[License](LICENSE)
