#!/bin/bash

# Install rustfmt
if [ "$RUSTFMT" == true ]; then
    rustup component add rustfmt
fi
