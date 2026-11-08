use std::io;


#[path = "lib/install_pacman.rs"]
pub mod install_pacman;
#[path = "lib/install_yay.rs"]
pub mod install_yay;


pub fn install() {
    let mut input = String::new();

    println!("Pacman or Yay: ");

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();

    if input == "pacman" || input == "Pacman" {
        install_pacman::install();
    }
    else if input == "yay" || input == "Yay" {
        install_yay::install();
    }
}