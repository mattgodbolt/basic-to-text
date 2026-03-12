# basic-to-text

BBC BASIC detokeniser written in Rust. Converts tokenised BBC BASIC binary
files back to readable text.

## Build & Test

```sh
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
```

## Project Layout

- `src/main.rs` — CLI entry point (clap)
- `src/lib.rs` — Core decoding logic and unit tests
- `src/tokens.rs` — Token tables (base, BASIC 2, ESCFN/ESCCOM/ESCSTMT)
- `src/error.rs` — Error types (thiserror)
- `tests/integration.rs` — Integration tests building binary programs in code

## Token encoding notes

- BBC BASIC tokenises keywords even inside quoted strings. The detokeniser
  must expand them back — do not skip token expansion inside `"..."`.
- After `REM` (0xF4) and `DATA` (0xDC), the rest of the line is literal bytes.
- `0x8D` is a line number marker followed by 3 encoded bytes (GOTO/GOSUB targets).
- In BASIC V, `0xC6`/`0xC7`/`0xC8` are two-byte extended token prefixes.
  In BASIC 2, these same bytes are single-byte tokens (AUTO, DELETE, LOAD).

## Test data

Real-world test files from `mattgodbolt/FinalLook` and `mattgodbolt/IRClient`
on GitHub. The FinalLook repo has `.bas` reference files that the tool's output
must match exactly.
