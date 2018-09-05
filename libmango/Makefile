SHELL := /usr/bin/env bash
EXAMPLE_SRC :=  $(shell find ./examples -name '*.c')
EXAMPLES := $(shell find ./examples -name '*.c' |  sed  's/\.c//g' | sed 's/s\//s\/build\//g')

build: rust

clean: rust_clean examples_clean

rust:
	cargo build

rust_clean:
	cargo clean


examples/build/%: examples/%.c
	mkdir -p examples/build
	gcc $< -o $@ -Isrc  -L. -l:./target/debug/libmango.so

examples: rust  $(EXAMPLES)

examples_clean:
	rm -rf examples/build


