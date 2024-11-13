mod file_manager;
mod password_generator;
mod system_monitor;

use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use sys_locale::get_locale;

#[derive(Debug)]
struct Messages {
    menu_prompt: &'static str,
    options: [&'static str; 5],
    enter_prompt: &'static str,
    invalid_input: &'static str,
    exit_message: &'static str,
}

fn get_language() -> Messages {
    let lang = get_locale().unwrap_or_else(|| "en".to_string());
    if lang.contains("ru") {
        Messages {
            menu_prompt: "Выберите действие:",
            options: [
                "1. Генерация пароля",
                "2. Сортировка файлов по расширению",
                "3. Удаление временных файлов",
                "4. Мониторинг системы",
                "0. Выход",
            ],
            enter_prompt: "Нажмите Enter, чтобы вернуться в меню.",
            invalid_input: "Неверный ввод, попробуйте снова.",
            exit_message: "Завершение программы...",
        }
    } else {
        Messages {
            menu_prompt: "Choose an action:",
            options: [
                "1. Generate Password",
                "2. Organize Files by Extension",
                "3. Delete Temporary Files",
                "4. System Monitoring",
                "0. Exit",
            ],
            enter_prompt: "Press Enter to return to menu.",
            invalid_input: "Invalid input, please try again.",
            exit_message: "Exiting program...",
        }
    }
}
fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
fn main() {
    let messages = get_language();
    loop {
        clear_console();
        println!("Системный язык: {}", get_locale().unwrap_or("не определен".to_string())); // Повторный вывод системного языка перед меню
        println!("{}", messages.menu_prompt);
        for option in &messages.options {
            println!("{}", option);
        }
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                clear_console();
                println!("Enter password length:");
                let mut length_input = String::new();
                io::stdin().read_line(&mut length_input).expect("Failed to read input");
                let length = length_input.trim().parse::<usize>().unwrap_or(12);
                let password = password_generator::generate_password(length);
                println!("Generated password: {}", password);
                println!("{}", messages.enter_prompt);
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "2" => {
                clear_console();
                println!("Enter directory path for organization:");
                let mut directory = String::new();
                io::stdin().read_line(&mut directory).expect("Failed to read input");
                file_manager::organize_files_by_extension(directory.trim()).expect("Error organizing files");
                println!("Files organized by extension.");
                println!("{}", messages.enter_prompt);
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "3" => {
                clear_console();
                println!("Enter directory path for deleting temporary files:");
                let mut directory = String::new();
                io::stdin().read_line(&mut directory).expect("Failed to read input");
                file_manager::delete_temp_files(directory.trim()).expect("Error deleting temporary files");
                println!("Temporary files deleted.");
                println!("{}", messages.enter_prompt);
                io::stdin().read_line(&mut String::new()).unwrap();
            },
            "4" => {
                system_monitor::monitor_system();
            },
            "0" => {
                clear_console();
                println!("{}", messages.exit_message);
                break;
            },
            _ => {
                println!("{}", messages.invalid_input);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}
