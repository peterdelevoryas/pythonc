#!/bin/sh

rustup target add i686-unknown-linux-gnu
cargo build --target=i686-unknown-linux-gnu
echo "File: "
file ./target/i686-unknown-linux-gnu/debug/rust-python
cp ./target/i686-unknown-linux-gnu/debug/rust-python .
zip -r cog.zip build.rs Cargo.* Makefile make_zip.sh pyyc README.md runtime rust-python src
