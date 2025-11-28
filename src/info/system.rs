use std::fs;

pub fn get_os() -> String {
    // Read /etc/os-release
    let contents = fs::read_to_string("/etc/os-release")
        .unwrap_or_else(|_| "NAME=Unknown".to_string());

    // Look for PRETTY_NAME first (best looking)
    for line in contents.lines() {
        if line.starts_with("PRETTY_NAME=") {
            return line
                .trim_start_matches("PRETTY_NAME=")
                .trim_matches('"')
                .to_string();
        }

        if line.starts_with("NAME=") {
            return line
                .trim_start_matches("NAME=")
                .trim_matches('"')
                .to_string();
        }
    }

    "Unknown Linux Distro".to_string()
}


