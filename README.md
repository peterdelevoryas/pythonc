# pythonc

A compiler for a subset of Python, implemented in Rust.

# Building

```bash
# install rust with rustup
curl https://sh.rustup.rs -sSf | sh

# clone repository
git clone https://github.com/csci4555-f17/pyyc-rust-python.git
cd ./pyyc-rust-python

# builds cog.zip, places pythonc in working directory
# by default, builds for i686-unknown-linux-gnu
make pythonc

# view usage
./pythonc -h
```

# Testing against [`pyyc-tests-contrib`](https://github.com/csci4555-f17/pyyc-tests-contrib)

```bash
# while in pyyc-rust-python directory
git submodule update --init
cargo test
```
