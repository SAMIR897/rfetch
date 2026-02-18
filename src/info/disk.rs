use std::process::Command;

pub fn get_disk() -> String {
    if cfg!(target_os = "macos") {
        let output = run_cmd("df", &["-H", "/"]);
        // Parse: Filesystem   Size   Used  Avail  Capacity  Mounted on
        //        /dev/disk3s1  500G   250G   250G     50%    /
        if let Some(line) = output.lines().nth(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let used = parts[2];
                let total = parts[1];
                let pct = parts[4];
                return format!("{} / {} ({})", used, total, pct);
            }
        }
    } else {
        // Linux: same df command works
        let output = run_cmd("df", &["-h", "/"]);
        if let Some(line) = output.lines().nth(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let used = parts[2];
                let total = parts[1];
                let pct = parts[4];
                return format!("{} / {} ({})", used, total, pct);
            }
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
