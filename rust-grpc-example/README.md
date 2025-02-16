# Rust gRPC example

grpc-rust-example/
│── src/
│   ├── main.rs       
│   ├── server.rs     # gRPC server implementation
│   ├── client.rs     # gRPC client implementation
│── proto/
│   ├── calc.proto    # Protobuf definition
│── Cargo.toml        # Rust project dependencies
│── build.rs          # gRPC code generation script


## Usage

```
cargo build
cargo run --bin server
cargo run --bin client
```