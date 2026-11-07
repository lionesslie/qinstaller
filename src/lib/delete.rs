use std::io;
use std::process::Command;

pub fn delete() {
    let mut input = String::new();

    println!("Pacman or Yay: ");

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();

    if input == "pacman" || input == "Pacman" {
        
        println!("Package: ");

        let mut package = String::new();
        io::stdin()
            .read_line(&mut package)
            .unwrap();
        let package = package.trim();

        Command::new("pacman")
            .arg("-Rns")
            .arg(package)
            .status()
            .expect("Failed");
    }
    else if input == "yay" || input == "Yay" {

        println!("Package: ");

        let mut package = String::new();
        io::stdin()
            .read_line(&mut package)
            .unwrap();
        let package = package.trim();

        Command::new("yay")
            .arg("-Rns")
            .arg(package)
            .status()
            .expect("Failed");
    }
}