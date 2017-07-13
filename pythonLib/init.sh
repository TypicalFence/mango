cd ../formatInterface
cargo build --release
cd ../pythonLib
cp ../formatInterface/target/release/libmango.so libmango.so
