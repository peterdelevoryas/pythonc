THIS_DIR=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

RUNTIME_ROOT=runtime
RUNTIME_DIR=$(THIS_DIR)/$(RUNTIME_ROOT)
RUNTIME_LIBFILE=libpyyruntime.a
RUNTIME_LIB=$(RUNTIME_DIR)/libpyyruntime.a

PYYC=$(THIS_DIR)/pyyc

CC=gcc
CFLAGS=-m32 -g -lm

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
