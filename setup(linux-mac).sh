#!/bin/bash

# Check if Rust is installed
if ! command -v rustc &> /dev/null
then
    echo "Rust is not installed. Please install Rust from https://www.rust-lang.org/"
    exit 1
fi

# Create a new Rust project
cargo new stellar_reality
cd stellar_reality

# Add dependencies to Cargo.toml
echo "
rand = \"0.8.5\"
termion = \"1.5.6\"
" >> Cargo.toml

echo "Setup complete! To start playing the game:"
echo "1. Copy the game code into src/main.rs"
echo "2. Run 'cargo run' in the stellar_reality directory"