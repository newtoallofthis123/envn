use clap::Parser;
use correct_word::{correct_word, Algorithm::Levenshtein};
use file::join_app_path;

#[derive(Parser, Debug)]
#[command(name="evnv", author="Ishan Joshi", version, about="Quickly write env's efficiently", long_about = None)]

/// The Args struct is used to parse the command line arguments
/// In order to make the command line arguments more user friendly
/// the user has the option to not pass in the command name
/// If the user does not pass in the command name, then the program
/// will prompt the user to enter the command name
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
mod utils;

/// Gets the command line arguments
fn get_args() -> Args {
    Args::parse()
}

/// Directly print a cool splash screen
pub fn print_splash_screen() {
    bunt::println!("{$blue}+-+-+-+-+-+-+{/$}");
    bunt::println!("{$green}|🔒|{/$}E|V|N|V|");
    bunt::println!("{$yellow}+-+-+-+-+-+-+{/$}");
}

fn main() {
    let args = get_args();

    print_splash_screen();

    let config = file::get_config_file();

    if args.cmd.clone().unwrap_or("".to_string()) == "help" {
        utils::display_help(args.name);
        std::process::exit(0);
    }

    // for ux, we make sure that the password file exists
    // before we do anything else

    if !join_app_path("auth").exists() {
        bunt::println!("{$red}No password file found{/$}");
        let password_confirm = inquire::Confirm::new("Do you want to create a password file?")
            .with_default(true)
            .prompt()
            .unwrap();
        if !password_confirm {
            bunt::println!("{$red}You have to have security!{/$}");
            std::process::exit(1);
        }

        file::set_password();
        std::process::exit(0);
    }

    // Small piece of code that checks if the user
    // has entered the correct password
    if config.ask_for_password && !utils::check_password() {
        bunt::println!("{$red}Error with Password:({/$}");
        std::process::exit(1);
    }

    let mut cmd = args.cmd;

    let accepted = vec![
        "add", "show", "save", "all", "load", "get", "edit", "delete", "reset",
    ];

    if cmd.is_none() {
        cmd = Some(
            inquire::Select::new("Enter a command", accepted.clone())
                .prompt()
                .unwrap()
                .to_string(),
        );
    }

    // If the user enters an invalid command, we try to
    // predict the correct command
    // We do this by using the Levenshtein algorithm with a threshold of 1
    // This threshold ensures that we only predict the correct command and not something
    // that is completely different
    if !accepted.contains(&cmd.clone().unwrap().as_str()) {
        let predicted = correct_word(Levenshtein, &cmd.clone().unwrap(), accepted, Some(1));
        if let Some(word) = predicted.word {
            cmd = Some(
                inquire::Text::new(&format!("Did you mean {}?", word))
                    .with_default(&word)
                    .prompt()
                    .unwrap(),
            );
            bunt::println!("{$yellow}Using {} instead{/$}", cmd.clone().unwrap());
        } else {
            bunt::println!("{$red}Invalid command{/$}");
            std::process::exit(1);
        }
    }
    db::prepare_db();
    commands::handle_command(&cmd.unwrap(), args.name);
}
