use std::env;

fn main() {
    let args = env::args();

    // Iterators in Rust consume values when they iterate
    for arg in args {
        // `args` moved here, can't use later
        println!("Arg {arg}");
    }
    // println!("#Args {}", args.count()); // ERROR using `args`

    // `env::args()` re-fetches the arguments from the OS each time it is called,
    // creating a new iterator. That is why we can use it again.
    let args: Vec<String> = env::args().collect();
    println!("All command-line args\n{:?}", args);
    for (i, arg) in args.iter().enumerate() {
        println!("Arg {i}: {arg}");
    }
}
