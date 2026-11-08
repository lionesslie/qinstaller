use std::io;

#[path = "backend/pacman/search_pacman.rs"]
pub mod search_pacman;
#[path = "backend/yay/search_yay.rs"]
pub mod search_yay;


pub fn search() {
    let mut input = String::new();

    println!("Pacman or Yay: ");

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();

    if input == "pacman" || input == "Pacman" {
        search_pacman::search();
    }
    else if input == "yay" || input == "Yay" {
        search_yay::search();
    }
}