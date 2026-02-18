use std::process::Command;

pub fn get_gpu() -> String {
    if cfg!(target_os = "macos") {
        // Use system_profiler
        let output = run_cmd("system_profiler", &["SPDisplaysDataType"]);
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Chipset Model:") || trimmed.starts_with("Chip:") {
                return trimmed.split(':').nth(1).unwrap_or("Unknown").trim().to_string();
            }
        }
        // Fallback: use CPU name (Apple Silicon is unified)
        run_cmd("sysctl", &["-n", "machdep.cpu.brand_string"])
    } else {
        // Linux: try lspci
        let output = run_cmd("lspci", &[]);
        for line in output.lines() {
            if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                if let Some(name) = line.split(':').last() {
                    return name.trim().to_string();
                }
            }
        }
        "Unknown".into()
    }
}

pub fn get_gpu_driver() -> String {
    if cfg!(target_os = "macos") {
        // Check for Metal support
        let output = run_cmd("system_profiler", &["SPDisplaysDataType"]);
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Metal") {
                return trimmed.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
        String::new()
    } else {
        String::new()
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
