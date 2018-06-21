gcc -o ./check check_test.c -Isrc  -L. -l:target/release/libmango.so -pthread -lcheck_pic -pthread -lrt -lm -lsubunit
