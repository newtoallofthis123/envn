pub fn handle_command(cmd: &str) {
    match cmd.to_string() {
        _ => bunt::println!("{$red}Command Not Found{/$}\nUse {$yellow}envn help{/$} to see available commands")
    }
}
