async fn async_func() {
    println!("Hello from async function!");
    println!("Task 1");
    println!("Task 2");
}

#[tokio::main]
async fn main() {
    let result = async_func(); // returns a Future, does NOT run
    result.await; // run the Future
}
