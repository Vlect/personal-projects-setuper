use std::io;
use std::fs;

fn main() {
    let mut directory_name = String::new();

    println!("Enter your name: ");

    io::stdin().read_line(&mut directory_name).expect("Failed to read line");

    fs::create_dir(directory_name.trim()).expect("Failed to create directory");
}
