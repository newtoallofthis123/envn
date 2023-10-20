use clap::Parser;
use file::get_path;

#[derive(Parser, Debug)]
#[command(name="evnv", author="Ishan Joshi", version, about="Quickly write env's efficiently", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required = false)]
    cmd: Option<String>,

    #[arg(required = false)]
    name: Option<String>,
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

    // for ux, we make sure that the password file exists
    // before we do anything else

    let mut flag = false;

    if !get_path("auth").exists() {
        bunt::println!("{$red}No password file found{/$}");
        let password_confirm = inquire::Confirm::new("Do you want to create a password file?")
            .with_default(true)
            .prompt()
            .unwrap();
        if !password_confirm {
            bunt::println!("{$red}You have to have security!{/$}");
            std::process::exit(1);
        }

        flag = file::set_password();
    }

    // Small piece of code that checks if the user
    // has entered the correct password
    if !utils::check_password() && !flag {
        bunt::println!("{$red}Error with Password:({/$}");
        std::process::exit(1);
    }

    let cmd = args.cmd;

    if cmd.is_none() {
        inquire::Select::new("Enter a command", vec!["set", "get", "add", "load", "all"])
            .prompt()
            .unwrap()
            .to_string();
    }

    db::prepare_db();

    commands::handle_command(&cmd.unwrap(), args.name);
}
