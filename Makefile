THIS_DIR=$(shell dirname $(lastword $(MAKEFILE_LIST)))

RUNTIME_ROOT=runtime
RUNTIME_DIR=$(THIS_DIR)/$(RUNTIME_ROOT)
RUNTIME_LIBFILE=libpyyruntime.a
RUNTIME_LIB=$(RUNTIME_DIR)/libpyyruntime.a

PYYC=$(THIS_DIR)/pyyc

CC=gcc
CFLAGS=-m32 -g -lm

TARGET=i686-unknown-linux-gnu
BUILD=debug
PYTHONC=$(THIS_DIR)/target/$(TARGET)/$(BUILD)/pythonc
COG_ZIP=$(THIS_DIR)/target/$(TARGET)/$(BUILD)/cog.zip

.PHONY: pythonc cog install test pyyruntime

# Create x86 assembly .s file using your compiler.
%.s: %.py
	$(PYYC) $<
	
# Create executable from your assembly .s file.
%: %.s $(RUNTIME_LIB)
	$(CC) $(CFLAGS) $< $(RUNTIME_LIB) -o $@
	
# Run your executable to create an output .out file given a .in input file.
%.out: %.in %
	cat $< | $* >$@
	
# Create the run-time library if necessary.
$(RUNTIME_ROOT)/$(RUNTIME_LIBNAME):
	$(MAKE) -C $(RUNTIME_DIR)

cog: $(COG_ZIP)

pythonc: $(PYTHONC)

$(COG_ZIP): $(PYYC) $(PYTHONC) Makefile runtime
	zip -r $(COG_ZIP) $(PYYC) Makefile runtime
	zip -j $(COG_ZIP) $(PYTHONC)

$(PYTHONC): src crates
	rustup target add i686-unknown-linux-gnu
	cargo build

test: pyyruntime
	git submodule update --init
	cargo test
	cargo test -p python-token
	cargo test -p python-ast
	cargo test -p python-ir
	cargo test -p python-trans

pyyruntime: runtime/
	make -C $(RUNTIME_ROOT)

install:
	cargo install
