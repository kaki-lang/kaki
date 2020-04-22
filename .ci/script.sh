#!/bin/bash

# Run rustfmt
if [ "$RUSTFMT" == true ]; then
  - cargo fmt --verbose --all -- --check
    exit
fi

# Run tests
cargo test --verbose --all
