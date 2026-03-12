# basic-to-text

Detokenise BBC BASIC binary files to readable text. Supports both BBC BASIC 2
(as found on the BBC Micro) and BBC BASIC V (as found on RISC OS / Acorn
Archimedes).

## Installation

### From source

```sh
cargo install --path .
```

### Pre-built binaries

Download from [GitHub Releases](../../releases) for Linux, macOS, and Windows.

## Usage

```
basic-to-text [OPTIONS] [INPUT]
```

| Option | Description |
|--------|-------------|
| `INPUT` | Input file (reads from stdin if omitted) |
| `-o`, `--output FILE` | Output file (writes to stdout if omitted) |
| `--basic2` | Use BBC BASIC 2 token set |
| `--basicv` | Use BBC BASIC V token set (default) |
| `-n`, `--line-numbers` | Prefix lines with line numbers |

### Examples

```sh
# Detokenise a BASIC V file
basic-to-text MyProgram,ffb

# Detokenise a BBC Micro BASIC 2 file with line numbers
basic-to-text --basic2 -n HELLO -o hello.bas

# Pipe from stdin
cat MyProgram | basic-to-text -o output.bas
```

## Features

- Decodes GOTO/GOSUB encoded line number references
- Handles BASIC V extended tokens (ESCFN, ESCCOM, ESCSTMT prefixes)
- Correctly expands tokenised keywords inside quoted strings
- Preserves literal bytes after REM and DATA statements
- Outputs UTF-8 (Latin-1 bytes mapped to their Unicode equivalents)

## Background

BBC BASIC stores programs in a compact tokenised binary format where keywords
like `PRINT`, `GOTO`, and `IF` are replaced with single bytes (or two-byte
sequences in BASIC V). Line number references in `GOTO` and `GOSUB` statements
are encoded as 3-byte sequences. This tool reverses that process to produce
human-readable text.

This is a Rust rewrite of `BBCBasicToText.py`, with added support for BBC BASIC
2, GOTO/GOSUB line number decoding, and proper handling of tokens inside strings
and after REM statements.

## License

(c) Matt Godbolt. Use however you like, as long as you put credit where credit's
due. Some information obtained from source code from
[RISC OS Open](https://www.riscosopen.org/).
