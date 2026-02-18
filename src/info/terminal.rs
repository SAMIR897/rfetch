pub fn get_terminal() -> String {
    // Check $TERM_PROGRAM first (most reliable)
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        return term;
    }

    // Check $TERMINAL
    if let Ok(term) = std::env::var("TERMINAL") {
        return term;
    }

    // Fallback to $TERM
    std::env::var("TERM").unwrap_or("Unknown".into())
}

pub fn get_terminal_font() -> String {
    // This is terminal-specific and hard to detect generically.
    // Check common terminals
    let term = get_terminal();

    match term.as_str() {
        "Apple_Terminal" => {
            // Try defaults read
            let output = std::process::Command::new("defaults")
                .args(["read", "com.apple.Terminal", "Default Window Settings"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();
            if !output.is_empty() {
                return format!("{} (profile)", output);
            }
        }
        "iTerm.app" | "iTerm2" => {
            return "iTerm2 Profile Font".into();
        }
        _ => {}
    }

    String::new()
}
