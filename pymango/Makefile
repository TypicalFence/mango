rust:
	cd  ../libmango && cargo build

install: rust
	pip3 install -e . --user

clean:
	cargo clean
	pip3 uninstall pymango

test:
	LD_LIBRARY_PATH=../libmango/target/debug pytest -v
