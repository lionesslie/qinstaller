use std::io;
use std::process::Command;


mod lib{
    pub mod install_pacman;
    pub mod install_yay;
    pub mod delete;
}

fn main(){


    let mut input = String::new();
    
    println!("Pacman or Yay or Delete: ");

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim()
        .parse()
        .expect("Failed!");


    if input == "pacman" || input == "Pacman" {
        lib::install_pacman::install();   
    }
    else if input == "yay" || input == "Yay" {
        lib::install_yay::install();
    }
    else if input == "delete" || input == "Delete" {
        lib::delete::delete();
    }
}