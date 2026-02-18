use std::process::Command;

pub fn get_cpu() -> String {
    if cfg!(target_os = "macos") {
        let brand = run_cmd("sysctl", &["-n", "machdep.cpu.brand_string"]);
        if !brand.is_empty() {
            // Clean up the string (remove extra spaces)
            brand.split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            "Unknown".into()
        }
    } else if cfg!(target_os = "windows") {
        let name = run_cmd("wmic", &["cpu", "get", "name"]);
        name.lines().nth(1).unwrap_or("Unknown").trim().to_string()
    } else {
        // Linux: read /proc/cpuinfo
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("model name") {
                    if let Some(name) = line.split(':').nth(1) {
                        return name.trim().to_string();
                    }
                }
            }
        }
        "Unknown".into()
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
