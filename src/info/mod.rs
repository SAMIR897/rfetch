pub mod os;
pub mod host;
pub mod uptime;
pub mod memory;
pub mod cpu;
pub mod gpu;
pub mod shell;
pub mod packages;
pub mod terminal;
pub mod resolution;
pub mod desktop;
pub mod theme;
pub mod network;
pub mod battery;
pub mod disk;
pub mod users;
pub mod media;
pub mod colors;

use colored::*;

/// Collects all info fields in Neofetch order and returns them as formatted strings.
pub fn collect_all() -> Vec<String> {
    let username = get_username();
    let hostname = sysinfo::System::host_name().unwrap_or("unknown".into());

    let mut lines: Vec<String> = Vec::new();

    // Title
    lines.push(format!("{}@{}", username.green().bold(), hostname.green().bold()));

    // Separator (match title length)
    let sep_len = username.len() + 1 + hostname.len();
    lines.push("-".repeat(sep_len));

    // OS
    lines.push(format!("{}: {}", "OS".cyan().bold(), os::get_os()));

    // Host
    lines.push(format!("{}: {}", "Host".cyan().bold(), host::get_host()));

    // Kernel
    let kernel = sysinfo::System::kernel_version().unwrap_or("Unknown".into());
    lines.push(format!("{}: {}", "Kernel".cyan().bold(), kernel));

    // Uptime
    lines.push(format!("{}: {}", "Uptime".cyan().bold(), uptime::get_uptime()));

    // Packages
    lines.push(format!("{}: {}", "Packages".cyan().bold(), packages::get_packages()));

    // Shell
    lines.push(format!("{}: {}", "Shell".cyan().bold(), shell::get_shell()));

    // Resolution
    let res = resolution::get_resolution();
    if !res.is_empty() {
        lines.push(format!("{}: {}", "Resolution".cyan().bold(), res));
    }

    // DE
    lines.push(format!("{}: {}", "DE".cyan().bold(), desktop::get_de()));

    // WM
    lines.push(format!("{}: {}", "WM".cyan().bold(), desktop::get_wm()));

    // WM Theme
    let wm_theme = theme::get_wm_theme();
    if !wm_theme.is_empty() {
        lines.push(format!("{}: {}", "WM Theme".cyan().bold(), wm_theme));
    }

    // Theme
    lines.push(format!("{}: {}", "Theme".cyan().bold(), theme::get_theme()));

    // Icons
    lines.push(format!("{}: {}", "Icons".cyan().bold(), theme::get_icons()));

    // Terminal
    lines.push(format!("{}: {}", "Terminal".cyan().bold(), terminal::get_terminal()));

    // Terminal Font
    let term_font = terminal::get_terminal_font();
    if !term_font.is_empty() {
        lines.push(format!("{}: {}", "Terminal Font".cyan().bold(), term_font));
    }

    // CPU
    lines.push(format!("{}: {}", "CPU".cyan().bold(), cpu::get_cpu()));

    // GPU
    lines.push(format!("{}: {}", "GPU".cyan().bold(), gpu::get_gpu()));

    // Memory
    lines.push(format!("{}: {}", "Memory".cyan().bold(), memory::get_memory()));

    // --- Bonus Fields ---

    // GPU Driver
    let gpu_driver = gpu::get_gpu_driver();
    if !gpu_driver.is_empty() {
        lines.push(format!("{}: {}", "GPU Driver".cyan().bold(), gpu_driver));
    }

    // Disk
    lines.push(format!("{}: {}", "Disk".cyan().bold(), disk::get_disk()));

    // Battery
    let batt = battery::get_battery();
    if !batt.is_empty() {
        lines.push(format!("{}: {}", "Battery".cyan().bold(), batt));
    }

    // Local IP
    lines.push(format!("{}: {}", "Local IP".cyan().bold(), network::get_local_ip()));

    // Users
    lines.push(format!("{}: {}", "Users".cyan().bold(), users::get_users()));

    // Empty line before color bar
    lines.push(String::new());

    // Color Bar
    lines.push(colors::get_color_bar());

    lines
}

fn get_username() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".into())
}
