use std::process::Command;

pub fn get_resolution() -> String {
    if cfg!(target_os = "macos") {
        let output = run_cmd("system_profiler", &["SPDisplaysDataType"]);
        for line in output.lines() {
            let trimmed = line.trim();
            // Look for resolution line like "Resolution: 3024 x 1964 Retina"
            if trimmed.starts_with("Resolution:") || trimmed.starts_with("UI Looks like:") {
                return trimmed.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
    } else if cfg!(target_os = "windows") {
        let h = run_cmd("wmic", &["path", "Win32_VideoController", "get", "CurrentHorizontalResolution"]);
        let v = run_cmd("wmic", &["path", "Win32_VideoController", "get", "CurrentVerticalResolution"]);
        
        let h_val = h.lines().nth(1).unwrap_or("").trim();
        let v_val = v.lines().nth(1).unwrap_or("").trim();
        
        if !h_val.is_empty() && !v_val.is_empty() {
            return format!("{} x {}", h_val, v_val);
        }
    } else {
        // Linux: try xrandr
        let output = run_cmd("xrandr", &["--current"]);
        for line in output.lines() {
            if line.contains('*') {
                // Connected display with active resolution
                let res = line.trim().split_whitespace().next().unwrap_or("");
                if !res.is_empty() {
                    return res.to_string();
                }
            }
        }
    }
    String::new()
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
