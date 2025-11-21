use std::fs;

pub fn uptime() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/uptime") {
        if let Some(first) = contents.split_whitespace().next() {
            if let Ok(seconds) = first.parse::<u64>() {
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                return format!("{}h {}m", hours, minutes);
            }
        }
    }
"Unknown_Uptime".to_string()
}