https://magiclen.org/rust-c-library/
```bash
cc hello-world/hello.c -O3 -c -fPIC -o "$OUT_DIR/hello-world.o"
ar crus libhello-world.a hello-world.o
cp libhello.a /usr/local/lib/  
cargo run
```