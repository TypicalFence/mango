gcc -o ./check test_file.c -Isrc  -L. -l:../target/release/libmango.so -pthread -lcheck_pic -pthread -lrt -lm -lsubunit
