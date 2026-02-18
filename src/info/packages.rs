use std::process::Command;

pub fn get_packages() -> String {
    let mut counts: Vec<String> = Vec::new();

    // Homebrew (macOS / Linux)
    if let Ok(entries) = std::fs::read_dir("/usr/local/Cellar") {
        let count = entries.count();
        if count > 0 { counts.push(format!("{} (brew)", count)); }
    } else if let Ok(entries) = std::fs::read_dir("/opt/homebrew/Cellar") {
        let count = entries.count();
        if count > 0 { counts.push(format!("{} (brew)", count)); }
    }

    // Homebrew casks
    if let Ok(entries) = std::fs::read_dir("/opt/homebrew/Caskroom") {
        let count = entries.count();
        if count > 0 { counts.push(format!("{} (brew-cask)", count)); }
    }

    // pacman (Arch)
    let pacman = run_cmd("pacman", &["-Qq"]);
    if !pacman.is_empty() {
        let count = pacman.lines().count();
        counts.push(format!("{} (pacman)", count));
    }

    // dpkg (Debian/Ubuntu)
    if std::path::Path::new("/var/lib/dpkg/status").exists() {
        let output = run_cmd("dpkg-query", &["-f", ".\n", "-W"]);
        if !output.is_empty() {
            let count = output.lines().count();
            counts.push(format!("{} (dpkg)", count));
        }
    }

    // rpm (Fedora/RHEL)
    let rpm = run_cmd("rpm", &["-qa"]);
    if !rpm.is_empty() {
        let count = rpm.lines().count();
        counts.push(format!("{} (rpm)", count));
    }

    // snap
    let snap = run_cmd("snap", &["list"]);
    if !snap.is_empty() {
        let count = snap.lines().count().saturating_sub(1); // header line
        if count > 0 { counts.push(format!("{} (snap)", count)); }
    }

    // flatpak
    let flatpak = run_cmd("flatpak", &["list"]);
    if !flatpak.is_empty() {
        let count = flatpak.lines().count();
        if count > 0 { counts.push(format!("{} (flatpak)", count)); }
    }

    if counts.is_empty() {
        "0".into()
    } else {
        counts.join(", ")
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
