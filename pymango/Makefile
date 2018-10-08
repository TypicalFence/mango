rust:
	cd  ../libmango && cargo build

install: rust
	pip3 install -e . --user

clean:
	cargo clean
	pip3 uninstall pymango

test: rust 
	LD_LIBRARY_PATH=../libmango/target/debug pytest -v -n1

test_single: rust
	LD_LIBRARY_PATH=../libmango/target/debug pytest -v

debug:
	LD_LIBRARY_PATH=../libmango/target/debug gdb --args  python3 -m pytest -v
