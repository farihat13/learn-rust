use tokio::join;
use tokio::time::{sleep, Duration};

async fn task1() {
    sleep(Duration::from_secs(2)).await; // non-blocking sleep
    println!("Task 1 completed");
}

async fn task2() {
    sleep(Duration::from_secs(1)).await; // non-blocking sleep
    println!("Task 2 completed");
}

#[tokio::main]
async fn main() {
    join!(task1(), task2()); // runs in parallel
}
