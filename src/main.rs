use std::io;

mod module{
    pub mod install;
    pub mod delete;
}

fn main(){


    let mut input = String::new();
    
    println!("Install or Delete: ");

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();


    if input == "install" || input == "Install" {
        module::install::install();   
    }
    else if input == "delete" || input == "Delete" {
        module::delete::delete();
    }
}