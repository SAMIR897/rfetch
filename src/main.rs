mod logo;
mod info;

fn main() {
    // Collect all system info
    let fields = info::collect_all();

    // Get logo based on detected OS
    let os_name = info::os::get_os_name_raw();
    let logo_key = if os_name == "Darwin" {
        "Darwin".to_string()
    } else {
        // Try to detect the actual distro name for logo matching
        info::os::get_distro_id().unwrap_or(os_name)
    };
    let logo = logo::get_logo(&logo_key);
    let logo_lines: Vec<&str> = logo.lines().collect();

    // Calculate logo width (use widest line, accounting for ANSI codes is hard, so use fixed width)
    let logo_width = 38;

    let max_lines = std::cmp::max(logo_lines.len(), fields.len());

    println!(); // Top margin
    for i in 0..max_lines {
        let logo_part = if i < logo_lines.len() { logo_lines[i] } else { "" };
        let info_part = if i < fields.len() { &fields[i] } else { &String::new() };
        println!("{:<width$} {}", logo_part, info_part, width = logo_width);
    }
    println!(); // Bottom margin
}
