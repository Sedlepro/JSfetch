use std::fs;

pub fn battery() -> (u8, String) {
    let bases = ["/sys/class/power_supply/BAT0", "/sys/class/power_supply/BAT1"];

    for base in bases {
        let cap_path = format!("{}/capacity", base);
        let status_path = format!("{}/status", base);

        if let (Ok(cap), Ok(status)) = (
            fs::read_to_string(&cap_path),
            fs::read_to_string(&status_path),
        ) {
            let cap_num = cap.trim().parse::<u8>().unwrap_or(0);
            let status_text = status.trim().to_string();
            return (cap_num, status_text);
        }
    }

    (0, "No Battery".to_string())
}

