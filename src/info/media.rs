use std::process::Command;

pub fn get_song() -> String {
    if cfg!(target_os = "macos") {
        // Try Music.app (formerly iTunes)
        let output = Command::new("osascript")
            .args(["-e", "tell application \"Music\" to get {name, artist} of current track"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        if !output.is_empty() && !output.contains("error") {
            return output;
        }

        // Try Spotify
        let output = Command::new("osascript")
            .args(["-e", "tell application \"Spotify\" to get {name, artist} of current track"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        if !output.is_empty() && !output.contains("error") {
            return output;
        }
    } else {
        // Linux: try playerctl
        let output = Command::new("playerctl")
            .args(["metadata", "--format", "{{ artist }} - {{ title }}"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        if !output.is_empty() {
            return output;
        }
    }

    String::new() // No music playing
}
