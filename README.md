# bittorrent-rust

A BitTorrent client written from scratch in Rust — parses `.torrent` files,
talks to HTTP trackers, and speaks the BitTorrent peer protocol to download
files piece by piece.

This started as a solution to the
["Build Your Own BitTorrent"](https://app.codecrafters.io/courses/bittorrent/overview)
challenge on CodeCrafters, and has since grown into its own standalone
project.

## Building

Requires a recent stable Rust toolchain (`cargo`).

```sh
cargo build --release
```

## Running

```sh
./your_program.sh <command> [args]
```

or directly via cargo:

```sh
cargo run --release -- <command> [args]
```

## Status

Work in progress — see `src/main.rs` for the current set of supported
commands.
