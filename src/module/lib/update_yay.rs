use std::io;
use std::process::Command;

pub fn update() {
    let mut package = String::new();

    println!("Package: ");

    io::stdin()
        .read_line(&mut package)
        .unwrap();

    let package = package.trim();

    Command::new("yay")
        .arg("-Sy")
        .arg(package)
        .status()
        .expect("Failed!");
}