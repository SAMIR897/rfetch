use std::process::Command;

pub fn get_theme() -> String {
    if cfg!(target_os = "macos") {
        let style = run_cmd("defaults", &["read", "-g", "AppleInterfaceStyle"]);
        let accent = run_cmd("defaults", &["read", "-g", "AppleAccentColor"]);

        let mode = if style.contains("Dark") { "Dark" } else { "Light" };

        let accent_name = match accent.trim() {
            "-1" => "Graphite",
            "0" => "Red",
            "1" => "Orange",
            "2" => "Yellow",
            "3" => "Green",
            "5" => "Purple",
            "6" => "Pink",
            _ => "Blue", // default
        };

        format!("{} ({})", accent_name, mode)
    } else {
        // Linux: try gsettings
        let theme = run_cmd("gsettings", &["get", "org.gnome.desktop.interface", "gtk-theme"]);
        if !theme.is_empty() {
            theme.trim_matches('\'').to_string()
        } else {
            "Unknown".into()
        }
    }
}

pub fn get_icons() -> String {
    if cfg!(target_os = "macos") {
        "SF Symbols".into()
    } else {
        let icons = run_cmd("gsettings", &["get", "org.gnome.desktop.interface", "icon-theme"]);
        if !icons.is_empty() {
            icons.trim_matches('\'').to_string()
        } else {
            "Unknown".into()
        }
    }
}

pub fn get_wm_theme() -> String {
    if cfg!(target_os = "macos") {
        // macOS doesn't really have a WM theme
        String::new()
    } else {
        let theme = run_cmd("gsettings", &["get", "org.gnome.desktop.wm.preferences", "theme"]);
        if !theme.is_empty() {
            theme.trim_matches('\'').to_string()
        } else {
            String::new()
        }
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
