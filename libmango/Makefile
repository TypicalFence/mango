SHELL := /usr/bin/env bash
EXAMPLE_SRC :=  $(shell find ./examples -name '*.c')
EXAMPLES := $(shell find ./examples -name '*.c' |  sed  's/\.c//g' | sed 's/s\//s\/build\//g')
TEST_SRC := $(shell find ./test -name '*.c')
FEATURES=gzip, aes
PREFIX=/usr/local

.PHONY: build clean rust rust_clean examples examples_clean test test_clean docs docs_clean release install uninstall

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

docs:
	cd docs && doxygen

docs_clean:
	rm -rf docs/html

release:
	cargo build --release --features "$(FEATURES)"

install: target/release/libmango.so
	./scripts/createPkgConfig.sh $(PREFIX)
	cp libmango.pc $(PREFIX)/lib/pkgconfig
	cp target/release/libmango.so $(PREFIX)/lib/
	cp libmango.h $(PREFIX)/include/

uninstall:
	rm $(PREFIX)/lib/pkgconfig/libmango.pc
	rm $(PREFIX)/lib/libmango.so
	rm $(PREFIX)/include/libmango.h

