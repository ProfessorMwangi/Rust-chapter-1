use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter Your Name:");
    io::stdin().read_line(&mut input).expect("Failed to read the Line");
    println!("Hello {}.", input.trim())
}
