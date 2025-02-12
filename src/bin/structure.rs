#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    c: u8,
}

impl User {
    fn print_active_status(&self) {
        println!("inside print_active_status: {}", self.active);
    }

    fn toggle_active_status(&mut self) {
        self.active = !self.active;
        println!("toggled from {} to {}", !self.active, self.active);
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    user1.email.push_str(" !!");
    println!("{}", user1.email);

    println!("{:#?}", user1);

    user1.print_active_status();
    user1.toggle_active_status();
}
