use sysinfo::System;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn monitor_system() {
    let mut system = System::new_all();
    println!("Нажмите Enter, чтобы вернуться в меню.");
    let handle = thread::spawn(move || {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
    });

    loop {
        clear_console();
        system.refresh_all();

        println!("Использование процессора:");
        let global_cpu_usage = system.global_cpu_usage();
        println!("Общая загрузка CPU: {:.2}%", global_cpu_usage);

        println!("\nИспользование памяти:");
        let total_memory_gb = system.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used_memory_gb = system.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        println!("Всего памяти: {:.2} ГБ", total_memory_gb);
        println!("Использовано памяти: {:.2} ГБ", used_memory_gb);

        println!("\nНажмите Enter, чтобы вернуться в меню.");

        thread::sleep(Duration::from_secs(1));

        if handle.is_finished() {
            break;
        }
    }
}