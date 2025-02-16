# Async Programming in Rust

The async programming model of is based on `futures`, `async`/`await`, and `tasks`.


```rust
// Each line waits for the previous line to finish
fn sync_func() {
    println!("Task 1");
    println!("Task 2");
}
```

```rust
fn async_func() {
    println!("ATask 1");
    println!("ATask 2");
}

#[tokio::main]
async fn main() {
    // A async function must be called from an async context
    let result = async_func(); // Returns a Future, does NOT run
    // `await` works only in async context
    result.await; // Runs the Future
}
```

## Why Async?

We use async programming to make programs non-blocking during I/O or network communication.

```rust
fn main() {
    let data1 = blocking_fetch_data();  // Takes 3 sec
    let data2 = blocking_fetch_data();  // Takes 3 sec

    println!("Got data: {} {}", data1, data2); // Total 6 sec
}
```

```rust
#[tokio::main]
async fn main() {
    let future1 = async_fetch_data(); // Takes 3 sec
    let future2 = async_fetch_data(); // Takes 3 sec

    let (data1, data2) = tokio::join!(future1, future2);
    println!("Got data: {} {}", data1, data2); // Total ~3 sec
}
```

## Futures
A `Future` represents a value that might not be available yet, but will be computed asynchronously at some point in the future.

* It is a trait (`std::future::Future`)
* It must be **polled** to make progress. 
* The async runtime (like `tokio` or `async-std`) keeps polling the `Future` until it completes.
* While waiting, the thread can do other work instead of blocking.

Example: You order a pizza (start a `Future`), and the chef is preparing it (async computation). You are watching youtube in meantime (non-blocking execution). When the pizza is ready (Future **resolves**), the waiter brings it to you.

## Usage

```
cargo build
cargo run
cargo run --bin async-fileio    
cargo run --bin sync-fileio    
```