# pythonc

A compiler for a subset of Python, implemented in Rust.

# Building

```bash
# install rust with rustup (requires nightly, for now)
curl https://sh.rustup.rs -sSf | sh

# clone repository
git clone https://github.com/peterdelevoryas/pythonc.git
cd ./pythonc

# build and install
cargo install

# output a 32-bit x86 (only architecture supported right now) assembly file
printf "x = 1\nwhile x != 10:\n\tprint x\n\tx = x + 1\nprint x" > test.py
pythonc test.py
cat test.s

# build the runtime for outputting binary files
make -C runtime
# pythonc uses PYTHONC_RUNTIME env var, or --runtime flag to specify path
# to compiled runtime library.
export PYTHONC_RUNTIME=$(readlink -f ./runtime/libpyyruntime.a)

# compile a binary and run it
pythonc test.py --emit bin
./test.bin

# view an intermediate stage's output
pythonc test.py --stdout --emit raised

# view usage for more
pythonc -h
```
