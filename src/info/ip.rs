use ureq::Agent;
use std::time::Duration;

pub fn get_global_ip() -> String {
    // Timeout of 3s because we want to be fast
    let agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(3))
        .timeout_write(Duration::from_secs(3))
        .build();

    // Try a few reliable IP echo services
    let urls = [
        "https://api.ipify.org",
        "https://ifconfig.me/ip",
        "https://icanhazip.com",
    ];

    for url in urls {
        if let Ok(response) = agent.get(url).call() {
            if let Ok(ip) = response.into_string() {
                let ip_trimmed: String = ip.trim().to_string();
                if !ip_trimmed.is_empty() {
                    return ip_trimmed;
                }
            }
        }
    }

    "Unknown".to_string()
}
