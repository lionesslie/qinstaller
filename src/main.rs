use std::io;

mod module{
    pub mod install;
    pub mod delete;
    pub mod update;
}

fn main(){

    let mut input = String::new();
    
    println!(" Install or Delete or Update: ");
    

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let input = input.trim();


    if input == "install" || input == "Install" {
        module::install::install();   
    }
    else if input == "update" || input == "Update" {
        module::update::update();
    }
    else if input == "delete" || input == "Delete" {
        module::delete::delete();
    }
}