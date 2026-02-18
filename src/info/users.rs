use std::process::Command;

pub fn get_users() -> String {
    let output = Command::new("who")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let mut users: Vec<String> = output
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(|s| s.to_string())
        .collect();

    users.sort();
    users.dedup();

    if users.is_empty() {
        std::env::var("USER").unwrap_or("Unknown".into())
    } else {
        users.join(", ")
    }
}
