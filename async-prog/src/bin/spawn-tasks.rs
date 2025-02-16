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
    let handle1 = tokio::spawn(task1());
    let handle2 = tokio::spawn(task2());
    println!("Spawned tasks, task1 and task2 running concurrently");
    handle1.await.unwrap();
    handle2.await.unwrap();
    println!("All tasks completed");
}
