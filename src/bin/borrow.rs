fn main(){

    let mut s:String = String::from("hello");
    println!("{}", s);
    
    s.push_str(", world!");
    println!("{}", s);
    
    s = take_ownership(s);
    println!("{}", s);
    
    pass_by_immut_ref(&s);
    pass_by_immut_ref2(&s);
    println!("{}", s);
    
    //pass_by_mut_ref(&mut s);
    
    let (s, len) = return_tuple(s);
}


// fn take_ownership(str:String) {
//     println!("inside fn take_ownership {}", str);
// }

fn take_ownership(str:String) -> String {
    println!("inside fn take_ownership {}", str);
    let mut str = str;
    pass_by_mut_ref(&mut str);
    str
}

fn pass_by_immut_ref(str:&String) {
    println!("inside fn pass_by_immut_ref {}", str);
}

fn pass_by_immut_ref2(str:&String) {
    println!("inside fn pass_by_immut_ref2 {}", str);
}

fn pass_by_mut_ref(str:& mut String) {
    str.push_str(" hello, bd!");   
    println!("inside fn pass_by_mut_ref {}", str);
}

fn return_tuple(str:String) -> (String, usize) {
    let len = str.len();
    (str, len)
}