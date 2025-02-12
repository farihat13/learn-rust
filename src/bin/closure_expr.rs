#![allow(unused)]

fn ten_times<F>(f: F) where F: Fn(i32) {
    for index in 0..4 {
        f(index);
    }
}


fn my_closure<F>(f: F) where F: Fn(u8, u8) {
    for index in 0..4 {
        f(index, index+1);
    }
}

fn main() {

    my_closure(
        |i, j| println!("i*j {}",i*j)
    );

    my_closure(
        |i, j| -> () {
            println!("i+j {}",i+j);
            println!("i/j {}",i/j);
        }
    );
    
    ten_times(|j| println!("hello, {}", j));
    // With type annotations
    ten_times(|j: i32| -> () { 
        println!("hello i32, {}", j); 
    });
    
    let word = "konnichiwa".to_owned();
    ten_times(move |j| println!("{}, {}", word, j));
}
