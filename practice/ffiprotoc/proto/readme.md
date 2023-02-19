```bash
brew install protoc
cargo install protobuf-codegen
protoc --rust_out ../src basic.proto
protoc --rust_out ../src parameters.proto
protoc --cpp_out ../code basic.proto 
protoc --cpp_out ../code parameters.proto
```