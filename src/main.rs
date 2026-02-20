use std::io::{stdout, Write};
use crossterm::{cursor, terminal, ExecutableCommand};
use std::thread;
use std::time::Duration;
use colored::Colorize;
use regex::Regex;

mod logo;
mod info;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let live_mode = args.contains(&"--live".to_string());

    // 1. Initial Render
    let mut height = render_fetch();

    // 2. Live Mode Loop
    if live_mode {
        stdout().execute(cursor::Hide).unwrap();
        
        loop {
            // thread::sleep(Duration::from_secs(1)); // Net Speed calc sleeps for 1s
            
            // Move cursor up by the number of lines we printed
            if height > 0 {
                stdout().execute(cursor::MoveUp(height as u16)).unwrap();
                stdout().execute(cursor::MoveToColumn(0)).unwrap();
            }
            
            // Re-render and update height (in case it changes)
            height = render_fetch();
        }
    }
}

fn render_fetch() -> usize {
    // Collect all system info
    let mut fields = info::collect_all();

    // Get logo based on detected OS
    let os_name = info::os::get_os_name_raw();
    let logo_key = if os_name == "Darwin" {
        "Darwin".to_string()
    } else {
        info::os::get_distro_id().unwrap_or(os_name)
    };
    let logo = logo::get_logo(&logo_key);
    let logo_lines: Vec<&str> = logo.lines().collect();

    // Calculate maximum width of the logo (stripping ANSI codes)
    let ansi_regex = Regex::new(r"\x1B\[[0-9;]*m").unwrap();
    let logo_width = logo_lines.iter()
        .map(|line| ansi_regex.replace_all(line, "").len())
        .max()
        .unwrap_or(0) + 4; // Add padding

    let max_lines = std::cmp::max(logo_lines.len(), fields.len());

    println!(); 
    for i in 0..max_lines {
        let logo_part = if i < logo_lines.len() { logo_lines[i] } else { "" };
        let info_part = if i < fields.len() { &fields[i] } else { &String::new() };
        
        // Calculate visible length for padding
        let visible_len = ansi_regex.replace_all(logo_part, "").len();
        let padding = if logo_width > visible_len { logo_width - visible_len } else { 0 };

        println!("{}{}{}", logo_part, " ".repeat(padding), info_part);
    }
    println!(); 
    
    // Return total lines printed: max_lines + 2 (top/bottom empty lines)
    max_lines + 2
}
