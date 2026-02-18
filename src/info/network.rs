use std::process::Command;

pub fn get_local_ip() -> String {
    if cfg!(target_os = "macos") {
        // Get the active network interface IP
        let output = run_cmd("ipconfig", &["getifaddr", "en0"]);
        if !output.is_empty() {
            return output;
        }
        // Try en1 (WiFi on some Macs)
        let output = run_cmd("ipconfig", &["getifaddr", "en1"]);
        if !output.is_empty() {
            return output;
        }
    } else {
        // Linux: hostname -I
        let output = run_cmd("hostname", &["-I"]);
        if !output.is_empty() {
            return output.split_whitespace().next().unwrap_or("").to_string();
        }
    }
    "Unknown".into()
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
