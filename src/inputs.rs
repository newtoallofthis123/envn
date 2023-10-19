pub fn print_splash_screen(){
    bunt::println!("{$blue}+-+-+-+-+-+-+{/$}");
    bunt::println!("{$green}|🔒|{/$}{$white}E|V|N|V|{/$}");
    bunt::println!("{$yellow}+-+-+-+-+-+-+{/$}");
}

pub fn get_input(msg: &str, default: Option<String>) -> String {
    let text_input = inquire::Text::new(msg);

    if let Some(default) = &default {
        text_input.clone().with_default(default);
    }

    let input = text_input.prompt().expect(format!("Failed to get input for {}", msg).as_str());

    input
}

pub fn spinner(msg: Option<String>)-> indicatif::ProgressBar{
    // static end_msg
    let end_msg = match msg {
        Some(msg) => msg,
        None => "Done".to_string()
    };

    let pb = indicatif::ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    pb.set_style(
        indicatif::ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "⣾",
                "⣽",
                "⣻",
                "⢿",
                "⡿",
                "⣟",
                "⣯",
                "⣷",
                "(200 Ok)"
            ]),
    );
    pb.set_message(end_msg.clone());
    pb
}
