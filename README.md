# pythonc

A compiler for a subset of Python, implemented in Rust.

# Building

```bash
# install rust with rustup
curl https://sh.rustup.rs -sSf | sh

# clone repository
git clone https://github.com/csci4555-f17/pyyc-rust-python.git
cd ./pyyc-rust-python

# by default, builds for i686-unknown-linux-gnu
# binary is output in target/i686-unknown-linux-gnu/debug
make pythonc

# view usage
./pythonc -h

# cog.zip output in target/i686-unknown-linux-gnu/debug
make cog
```

# Testing against [`pyyc-tests-contrib`](https://github.com/csci4555-f17/pyyc-tests-contrib)

```bash
# while in pyyc-rust-python directory
# runs cargo test for root and subdirectory crates
make test
```
