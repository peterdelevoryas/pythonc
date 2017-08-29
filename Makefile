THIS_DIR=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

RUNTIME_ROOT=runtime
RUNTIME_DIR=$(THIS_DIR)/$(RUNTIME_ROOT)
RUNTIME_LIB=$(RUNTIME_ROOT)/libpyyruntime.a

PYYC=$(THIS_DIR)/pyyc

CC=gcc
CFLAGS=-m32 -g -lm -I$(RUNTIME_DIR) -L$(RUNTIME_DIR) -lpyyruntime

# Create x86 assembly .s file using your compiler.
%.s: %.py
	$(PYYC) $<
	
# Create executable from your assembly .s file.
%: %.s $(RUNTIME_LIB)
	$(CC) $(CFLAGS) $< $(wildcard $(RUNTIME_DIR)/*.o) -o $@
	
# Run your executable to create an output .out file given a .in input file.
%.out: %.in %
	cat $< | $* >$@
	
# Create the run-time library if necessary.
$(RUNTIME_LIB):
	$(MAKE) -C $(RUNTIME_DIR)
