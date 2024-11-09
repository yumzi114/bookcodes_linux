# bookcodes_linux

## bindc
gcc -c c_src/hello.c
gcc -shared -o libhello.so hello.o
LD_LIBRARY_PATH=. cargo run
