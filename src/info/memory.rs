use std::fs;

pub struct Memory {
    pub used_gb: f64,
    pub total_gb: f64,
}

pub fn get_mem() -> Memory {

    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        let mut total_kb = 0u64;
        let mut available_kb = 0u64;

        for line in content.lines() {
            if line.starts_with("MemTotal") {
                total_kb = extract_kb(line);
            }
            else if line.starts_with("MemAvailable") {
                available_kb = extract_kb(line);
            }
        }
        let used_kb = total_kb.saturating_sub(available_kb);

        return Memory {
            used_gb: kb_to_gb(used_kb),
            total_gb: kb_to_gb(total_kb),
        };
    }
    Memory {
        used_gb: 0.0,
        total_gb: 0.0,
    }
}
fn extract_kb(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
}

fn kb_to_gb(kb: u64) -> f64 {
    kb as f64 / 1024.0 / 1024.0
}
