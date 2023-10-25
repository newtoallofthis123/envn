pub fn print_splash_screen() {
    bunt::println!("{$blue}+-+-+-+-+-+-+{/$}");
    bunt::println!("{$green}|🔒|{/$}E|V|N|V|");
    bunt::println!("{$yellow}+-+-+-+-+-+-+{/$}");
}

pub fn _get_input(msg: &str, default: Option<String>) -> String {
    let text_input = inquire::Text::new(msg);

    if let Some(default) = &default {
        text_input.clone().with_default(default);
    }

    text_input
        .prompt()
        .unwrap_or_else(|_| panic!("Failed to get input for {}", msg))
}
