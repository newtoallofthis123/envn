use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="evnv", author="Ishan Joshi", version, about="Quickly write env's efficiently", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required = false, short)]
    cmd: Option<String>,
}

mod commands;
mod db;
mod encryption;
mod file;
mod inputs;
mod utils;

fn get_args() -> Args {
    Args::parse()
}

fn main() {
    let args = get_args();

    inputs::print_splash_screen();

    // Small piece of code that checks if the user
    // has entered the correct password
    if !utils::check_password() {
        bunt::println!("{$red}Error with Password:({/$}");
        std::process::exit(1);
    }

    let cmd = args
        .cmd
        .unwrap_or(inputs::get_input("Enter your command", None));

    commands::handle_command(&cmd);

    db::prepare_db();

    let env_to_insert = utils::construct_struct("cool".to_string(), "COOL_ONE".to_string(), "noice".to_string());

    db::insert_env(env_to_insert);

    let env = db::get_by_name("cool");

    let decrypted_env = utils::decrypt_struct(env);

    println!("{:?}", decrypted_env);
}
