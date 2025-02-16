use tokio::{
    task,
    time::{sleep, Duration},
};

fn sync_task() {
    println!("Task 1");
    println!("Task 2");
}

async fn async_task() {
    for n in 1..11 {
        println!("Async Task! {n}");
        sleep(Duration::from_millis(10)).await; // yield control to allow interleaving
    }
}

/// Interleaving sync and async tasks
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let result = async_task(); // returns a Future, does NOT run
    result.await; // run the Future

    // spawn async_task so it runs concurrently
    let handle = task::spawn(async_task());

    for _ in 0..5 {
        sync_task();
        sleep(Duration::from_millis(20)).await; // yield control to allow interleaving
    }

    // await the async task properly
    handle.await.expect("Async task failed");

    sync_task();
}
