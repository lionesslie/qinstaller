use std::io;


#[path = "lib/delete_pacman.rs"]
pub mod delete_pacman;
#[path = "lib/delete_yay.rs"]
pub mod delete_yay;


pub fn delete() {
    let mut input = String::new();

    println!("Pacman or Yay: ");

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();

    if input == "pacman" || input == "Pacman" {
        delete_pacman::delete();
    }
    else if input == "yay" || input == "Yay" {
        delete_yay::delete();
    }
}