use std::fs;
use std::process::Command;

pub fn get_gpu() -> String {
    // Raspberry Pi
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("Hardware") {
                let hw = line.split(':').nth(1).unwrap_or("").trim();
                let gpu = map_pi_gpu(hw);
                if gpu != "Unknown" {
                    return gpu;
                }
            }
        }
    }

    // PC Linux (clean)
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d'")
        .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        let mut gpus: Vec<String> = Vec::new();

        for line in text.lines() {
            let gpu = clean_gpu_line(line);
            if !gpu.is_empty() {
                gpus.push(gpu);
            }
        }

        if !gpus.is_empty() {
            return gpus.join(" + ");
        }
    }

    "Unknown GPU".to_string()
}

fn clean_gpu_line(line: &str) -> String {
    // ignorer les non-GPU
    if line.to_lowercase().contains("realtek")
        || line.to_lowercase().contains("card reader")
        || line.to_lowercase().contains("unassigned")
    {
        return "".to_string();
    }

    // enlever l'adresse PCI
    let after_addr = match line.splitn(2, ": ").nth(1) {
        Some(v) => v,
        None => return "".to_string(),
    };

    // enlever les (rev xx)
    let no_rev = match after_addr.split('(').next() {
        Some(v) => v,
        None => after_addr,
    };

    // nettoyer mots inutiles
    let mut cleaned = no_rev
        .replace("Corporation", "")
        .replace("controller", "")
        .replace("compatible", "")
        .trim()
        .to_string();

    // extraire NVIDIA entre crochets si présent
    if let Some(idx) = cleaned.find('[') {
        if let Some(end) = cleaned.find(']') {
            cleaned = cleaned[idx + 1..end].to_string();
        }
    }

    cleaned.trim().to_string()
}

fn map_pi_gpu(hw: &str) -> String {
    match hw {
        "BCM2835" => "Broadcom VideoCore IV".to_string(),
        "BCM2836" => "Broadcom VideoCore IV".to_string(),
        "BCM2837" => "Broadcom VideoCore IV".to_string(),
        "BCM2711" => "Broadcom VideoCore VI".to_string(),
        "BCM2712" => "Broadcom VideoCore VII".to_string(),
        _ => "Unknown".to_string(),
    }
}

