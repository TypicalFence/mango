rust:
	cargo build

install: rust
	pip3 install -e . --user

clean:
	cargo clean
	pip3 uninstall pymango

test: install
	pytest
