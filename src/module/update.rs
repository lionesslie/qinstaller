use std::io;

#[path = "lib/update_pacman.rs"]
pub mod update_pacman;
#[path = "lib/update_yay.rs"]
pub mod update_yay;


pub fn update() {
    let mut input = String::new();

    println!("Pacman or Yay: ");

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();

    if input == "pacman" || input == "Pacman" {
        update_pacman::update();
    }
    else if input == "yay" || input == "Yay" {
        update_yay::update();
    }
}