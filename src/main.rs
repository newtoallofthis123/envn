use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="evnv", author="Ishan Joshi", version, about="Quickly write env's efficiently", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required = false, short)]
    cmd: Option<String>,
}

mod commands;
mod encryption;
mod file;
mod inputs;
mod utils;
mod db;

fn get_args() -> Args {
    Args::parse()
}

fn main() {
    let args = get_args();

    inputs::print_splash_screen();

    // Small piece of code that checks if the user
    // has entered the correct password
    //if !utils::check_password() {
      //  bunt::println!("{$red}Error with Password:({/$}");
        //std::process::exit(1);
    //}

    //let cmd = args
      //  .cmd
        //.unwrap_or(inputs::get_input("Enter your command", None));

    //commands::handle_command(&cmd);
    
    let key = encryption::get_key();
    let nonce = encryption::get_none();

    let data = "Hello World";

    println!("Data: {:?}", data.as_bytes());

    let encrypted_data = encryption::encrypt(key, nonce.clone(), data.as_bytes());

    println!("Encrypted Data: {:?}", encrypted_data);

    let decrypted_data = encryption::decrypt(key, nonce.clone(), encrypted_data);

    println!("Decrypted Data: {:?}", decrypted_data);
}
