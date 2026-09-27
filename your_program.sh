#!/bin/sh
#
# Builds and runs the project locally.

set -e # Exit early if any commands fail

(
  cd "$(dirname "$0")" # Ensure compile steps are run within the repository directory
  cargo build --release --target-dir=/tmp/codecrafters-build-bittorrent-rust --manifest-path Cargo.toml
)

exec /tmp/codecrafters-build-bittorrent-rust/release/codecrafters-bittorrent "$@"
