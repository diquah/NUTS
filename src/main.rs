use std::path::PathBuf;

use dirs;

// Print and force stdout to flush
fn forceprint(data: &str) {
    print!("{}", data);
    std::io::Write::flush(&mut std::io::stdout()).expect("FATAL ERROR: Cannot use stdout");
}

// Requests a user's input with a prompt.
fn get_user_input() -> String {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("FATAL ERROR: Cannot get stdin.");

    return input.trim().to_string();
}

fn main() {
    // Set initial path of shell.
    let current_path: PathBuf = dirs::home_dir().unwrap();

    'shell: loop {
        forceprint(&format!("{} > ", current_path.display()));
        let input: String = get_user_input();
        let command_keyword: &str = input.split_whitespace().collect::<Vec<&str>>()[0];
        let command_info: &str =
            (" ".to_owned() + input.strip_prefix(command_keyword).unwrap()).as_str();

        match command_keyword {
            // Quit command is only to be enabled in non-release builds.
            #[cfg(debug_assertions)]
            "quit" => {
                forceprint("goodbye <3\n\n");
                break 'shell;
            }

            _ => {}
        }
    }
}
