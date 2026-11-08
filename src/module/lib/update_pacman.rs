use std::io;
use std::process::Command;

pub fn install() {
    let mut package = String::new();

    println!("Package: ");

    io::stdin()
        .read_line(&mut package)
        .unwrap();

    let package = package.trim();

    Command::new("pacman")
        .arg("-Sy")
        .arg(package)
        .status()
        .expect("Failed!");
}