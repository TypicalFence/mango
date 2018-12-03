SHELL := /usr/bin/env bash
EXAMPLE_SRC :=  $(shell find ./examples -name '*.c')
EXAMPLES := $(shell find ./examples -name '*.c' |  sed  's/\.c//g' | sed 's/s\//s\/build\//g')
TEST_SRC := $(shell find ./test -name '*.c')
FEATURES=gzip, aes

build: rust

clean: rust_clean examples_clean test_clean docs_clean

rust:
	cargo build --features "$(FEATURES)"

rust_clean:
	cargo clean

examples/build/%: examples/%.c
	mkdir -p examples/build
	gcc $< -o $@ -Isrc  -L. -l:./target/debug/libmango.so

examples: rust  $(EXAMPLES)

examples_clean:
	rm -rf examples/build

test: test/run_tests
	./test/run_tests

test_clean:
	rm ./test/run_tests

test/run_tests: rust $(TEST_SRC)
	gcc -o test/run_tests $(TEST_SRC) -Isrc -L. -l:./target/debug/libmango.so $(shell pkg-config --cflags --libs check)

documentation:
	cd docs && doxygen

docs_clean:
	rm -rf docs/html

release:
	cargo build --release --features "$(FEATURES)"

install: target/release/libmango.so
	cp target/release/libmango.so /usr/lib/
	cp libmango.h /usr/include/

uninstall:
	rm /usr/lib/libmango.so
	rm /usr/include/libmango.h

