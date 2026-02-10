# LOC
> LOC is short for lines of code.

A fast command-line line counter written in Rust.

It recursively scans a directory, groups files by detected language/type, and prints a summary table with:
- file count
- blank lines
- code lines
- total lines

## Requirements

- Rust toolchain (edition 2024; stable channel is recommended)

## Build

```bash
cargo build
```

## Run

Count lines in the current directory:

```bash
cargo run --release
```

Count lines in a specific directory:

```bash
cargo run --release -- --dir ./path/to/project
```

Show CLI help:

```bash
cargo run -- --help
```

## CLI

```text
Usage: loc [OPTIONS]

Options:
  -d, --dir <DIR>  Specify the directory
  -h, --help       Print help
  -V, --version    Print version
```

## What gets counted

- Traversal uses the `ignore` crate (`WalkBuilder`) and includes files discovered under the target directory.
- File type is inferred from filename/extension (for example `rs`, `ts`, `py`, `md`, `Dockerfile`, `Makefile`, etc.).
- Files that cannot be read as UTF-8 text are skipped.
- Blank lines are lines where `line.is_empty()` is `true` (whitespace-only lines are not considered blank).

## Development

Run checks:

```bash
cargo check
```

Run benchmarks:

```bash
cargo bench
```

## Project layout

```text
src/
  main.rs             # CLI entry point
  lib.rs              # crate modules
  cli/mod.rs          # argument parsing, file traversal, counting, table output
  types/
    cli.rs            # clap CLI struct
    error.rs          # error type definitions
    file_count.rs     # aggregate counters
    file_type.rs      # file-type enum and display names
benches/
  loc.rs              # criterion benchmark for count_lines
```
