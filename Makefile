# Makefile

# Default target
all: build

# Build the project for RISC-V
build:
	cargo build

# Clean the project
clean:
	cargo clean

.PHONY: all build run clean