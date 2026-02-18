use std::process::Command;

pub fn get_battery() -> String {
    if cfg!(target_os = "macos") {
        let output = run_cmd("pmset", &["-g", "batt"]);
        // Parse: "Now drawing from 'Battery Power'" and "InternalBattery-0 (id=...)   100%; charging;"
        for line in output.lines() {
            if line.contains("InternalBattery") {
                // Extract percentage and state
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    let info = parts[1].trim();
                    // info looks like: "100%; charging; 0:00 remaining present: true"
                    let pct = info.split(';').next().unwrap_or("").trim();
                    let state = info.split(';').nth(1).unwrap_or("").trim();

                    let state_str = match state {
                        s if s.contains("discharging") => "Not Plugged In",
                        s if s.contains("charged") => "Full",
                        s if s.contains("charging") => "Charging",
                        _ => "",
                    };

                    if !state_str.is_empty() {
                        return format!("{} [{}]", pct, state_str);
                    }
                    return pct.to_string();
                }
            }
        }
    } else {
        // Linux: /sys/class/power_supply/BAT0
        if let Ok(cap) = std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
            let status = std::fs::read_to_string("/sys/class/power_supply/BAT0/status")
                .unwrap_or("Unknown".into());
            return format!("{}% [{}]", cap.trim(), status.trim());
        }
    }
    String::new() // No battery (desktop)
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
