fn greet(s: String) -> String{
    println!("Hello {s}");
    s
}

fn greet_borrow(s: &String) {
    println!("Hello {s}");
}

fn greet_borrow_mut(s: &mut String) {
*s = format!("Hello {s}")
}

fn main() {
    let name = "Hello".to_string();
    let name = greet(name);
    let name = greet(name);
    greet_borrow(&name);
    let mut name = "Adam".to_string();
    greet_borrow_mut(&mut name);
    println!("{name}");
}
