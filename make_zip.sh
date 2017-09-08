#!/bin/sh

curl https://sh.rustup.rs -sSf | sh
$HOME/.cargo/bin/rustup target add i686-unknown-linux-gnu
$HOME/.cargo/bin/cargo build --target=i686-unknown-linux-gnu
cp ./target/i686-unknown-linux-gnu/debug/rust-python .
file ./rust-python
zip -r cog.zip build.rs Cargo.* Makefile make_zip.sh pyyc README.md runtime rust-python src
