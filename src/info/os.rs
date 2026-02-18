use std::process::Command;

/// Returns the raw OS name from sysinfo (e.g. "Darwin", "Linux")
pub fn get_os_name_raw() -> String {
    sysinfo::System::name().unwrap_or("Unknown".into())
}

/// Returns a human-readable OS string like "macOS Sequoia 15.3" or "Ubuntu 22.04"
pub fn get_os() -> String {
    let raw = get_os_name_raw();

    match raw.as_str() {
        "Darwin" => get_macos_name(),
        _ => get_linux_os(),
    }
}

fn get_macos_name() -> String {
    // Use sw_vers to get macOS version
    let version = run_cmd("sw_vers", &["-productVersion"]);
    let build = run_cmd("sw_vers", &["-buildVersion"]);

    let codename = match version.as_str() {
        v if v.starts_with("26.") => "macOS Tahoe",
        v if v.starts_with("15.") => "macOS Sequoia",
        v if v.starts_with("14.") => "macOS Sonoma",
        v if v.starts_with("13.") => "macOS Ventura",
        v if v.starts_with("12.") => "macOS Monterey",
        v if v.starts_with("11.") => "macOS Big Sur",
        v if v.starts_with("10.15") => "macOS Catalina",
        v if v.starts_with("10.14") => "macOS Mojave",
        v if v.starts_with("10.13") => "macOS High Sierra",
        v if v.starts_with("10.12") => "macOS Sierra",
        _ => "macOS",
    };

    // Get architecture
    let arch = run_cmd("uname", &["-m"]);

    format!("{} {} {} {}", codename, version, build, arch)
}

fn get_linux_os() -> String {
    // Try /etc/os-release
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let name = line.trim_start_matches("PRETTY_NAME=").trim_matches('"');
                let arch = run_cmd("uname", &["-m"]);
                return format!("{} {}", name, arch);
            }
        }
    }

    // Fallback
    let name = sysinfo::System::name().unwrap_or("Linux".into());
    let version = sysinfo::System::os_version().unwrap_or_default();
    let arch = run_cmd("uname", &["-m"]);
    format!("{} {} {}", name, version, arch)
}

/// Returns the distro ID from /etc/os-release for logo matching.
/// e.g. "Ubuntu", "Arch", "Fedora", "Debian"
pub fn get_distro_id() -> Option<String> {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("NAME=") {
                let name = line.trim_start_matches("NAME=").trim_matches('"');
                return Some(name.to_string());
            }
        }
    }
    None
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
