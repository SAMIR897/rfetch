use std::process::Command;

pub fn get_host() -> String {
    if cfg!(target_os = "macos") {
        get_macos_host()
    } else {
        get_linux_host()
    }
}

fn get_macos_host() -> String {
    // sysctl hw.model gives something like "Mac14,6"
    let model_id = run_cmd("sysctl", &["-n", "hw.model"]);

    // Map model IDs to human-readable names
    let friendly = match model_id.as_str() {
        m if m.starts_with("Mac14") => "MacBook Pro (M2)",
        m if m.starts_with("Mac15") => "MacBook Air (M3)",
        m if m.starts_with("Mac16") => "MacBook Pro (M4)",
        m if m.starts_with("MacBookPro18") => "MacBook Pro (M1 Pro/Max)",
        m if m.starts_with("MacBookPro17") => "MacBook Pro (M1)",
        m if m.starts_with("MacBookAir10") => "MacBook Air (M1)",
        m if m.starts_with("iMac") => "iMac",
        m if m.starts_with("Macmini") => "Mac mini",
        m if m.starts_with("MacPro") => "Mac Pro",
        _ => &model_id,
    };

    // Also try to get chip name
    let chip = run_cmd("sysctl", &["-n", "machdep.cpu.brand_string"]);
    if !chip.is_empty() && !friendly.contains("M1") && !friendly.contains("M2") && !friendly.contains("M3") && !friendly.contains("M4") {
        format!("{} ({})", friendly, chip)
    } else {
        friendly.to_string()
    }
}

fn get_linux_host() -> String {
    // Try /sys/devices/virtual/dmi/id/product_name
    if let Ok(name) = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
        let name = name.trim().to_string();
        if !name.is_empty() && name != "System Product Name" {
            // Also get version
            let version = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")
                .map(|v| v.trim().to_string())
                .unwrap_or_default();
            if !version.is_empty() && version != "System Version" {
                return format!("{} {}", name, version);
            }
            return name;
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
