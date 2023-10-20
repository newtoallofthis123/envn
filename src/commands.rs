use crate::utils::DisplayEnv;

pub fn handle_command(cmd: &str, name: Option<String>) {
    match cmd {
        "get" => get_command(name),
        "set" => set_command(),
        "file" => file_command(""),
        _ => bunt::println!("{$red}Command Not Found{/$}\nUse {$yellow}envn help{/$} to see available commands")
    }
}

fn set_command(){
    bunt::println!("The {$yellow}Setter{/$}");

    let key = inquire::Text::new("Secret Name").prompt().unwrap();
    let value = inquire::Text::new("Secret Value").prompt().unwrap();
    let name = inquire::Text::new("The identifier for this secret").prompt().unwrap();

    let env_entry = crate::utils::construct_struct(name, key, value);

    if crate::db::insert_env(env_entry){
        bunt::println!("{$green}Secret Saved{/$}");
    }
    else{
        bunt::println!("{$red}Secret Not Saved{/$}");
    }
}

fn get_command(name: Option<String>){
    bunt::println!("The {$yellow}Getter{/$}");

    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Secret Name").prompt().unwrap()
    };

    let env_entry = crate::db::get_by_name(&name);

    if env_entry.is_none(){
        bunt::println!("{$red}Secret Not Found{/$}");
        return;
    }

    let env_entry = env_entry.unwrap();

    let env = crate::utils::decrypt_struct(env_entry);

    crate::utils::display_env(env);
}

fn file_command(filename: &str){
    bunt::println!("The {$yellow}File{/$}");
    bunt::println!("Pressing enter will take you into add mode. Just press '!q' to exit add mode");

    let confirm = inquire::Confirm::new("Enter add mode").with_default(true).prompt().unwrap();
    if !confirm{
        return;
    }

    let envs = crate::db::get_all_names();
    let mut env_names = envs.iter().map(|env| env.name.clone()).collect::<Vec<String>>();
    let mut envs_to_write: Vec<DisplayEnv> = Vec::new();

    let flag = 0;

    while flag == 0{
        if env_names.len() == 0{
            break;
        }

        let to_add = inquire::Select::new("Select a secret to add", env_names.clone()).prompt().unwrap();
        
        if to_add == "!q"{
            break;
        }

        if !env_names.contains(&to_add){
            bunt::println!("{$red}Secret Not Found{/$}");
            continue;
        }

        let env = crate::db::get_by_name(&to_add).unwrap();
        let final_env = crate::utils::decrypt_struct(env);
        envs_to_write.push(final_env);
        
        env_names.remove(env_names.iter().position(|x| *x == to_add).unwrap());
    }

    println!("{:?}", envs_to_write);
}
