cargo build 
gcc -o ./wtf wtf.c -Isrc  -L. -l:../target/debug/libmango.so   
