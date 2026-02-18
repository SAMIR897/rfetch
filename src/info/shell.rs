use std::process::Command;

pub fn get_shell() -> String {
    let shell_path = std::env::var("SHELL").unwrap_or("/bin/sh".into());
    let shell_name = shell_path.split('/').last().unwrap_or("sh");

    // Try to get version
    let version = match shell_name {
        "zsh" => {
            run_cmd(&shell_path, &["--version"])
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .to_string()
        }
        "bash" => {
            run_cmd(&shell_path, &["--version"])
                .lines()
                .next()
                .and_then(|l| l.split("version ").nth(1))
                .and_then(|v| v.split('(').next())
                .unwrap_or("")
                .trim()
                .to_string()
        }
        "fish" => {
            run_cmd(&shell_path, &["--version"])
                .split("version ").nth(1)
                .unwrap_or("")
                .trim()
                .to_string()
        }
        _ => String::new(),
    };

    if version.is_empty() {
        shell_name.to_string()
    } else {
        format!("{} {}", shell_name, version)
    }
}

fn run_cmd(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
