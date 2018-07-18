gcc -o ./check test_file.c -Isrc  -L. -l:../target/debug/libmango.so -pthread -lcheck 
  
