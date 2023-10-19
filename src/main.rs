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

fn get_args() -> Args {
    Args::parse()
}

fn main() {
    let args = get_args();

    inputs::print_splash_screen();

    // Small piece of code that checks if the user
    // has entered the correct password
    if !utils::check_password() {
        bunt::println!("{$red}Wrong Password :({/$}");
        std::process::exit(1);
    }

    let cmd = args
        .cmd
        .unwrap_or(inputs::get_input("Enter your command", None));

    commands::handle_command(&cmd);
}
