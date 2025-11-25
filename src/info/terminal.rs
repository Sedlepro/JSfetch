use std::env;
use std::fs;

pub fn terminal() -> String {

    if let Ok(term) = env::var("TERM_PROGRAM") {
        return term;
    }

    if let Ok(term) = env::var("TERMINAL_EMULATOR") {
        return term;
    }

    let stat = fs::read_to_string("/proc/self/stat");
    if let Ok(contents) = stat {
        let parts: Vec<&str> = contents.split_whitespace().collect();
        if parts.len() > 3 {
            // ppid is the 4th field
            if let Ok(ppid) = parts[3].parse::<u32>() {
                let comm_path = format!("/proc/{}/comm", ppid);

                if let Ok(comm) = fs::read_to_string(comm_path) {
                    let name = comm.trim().to_string();

                    let known_terms = [
                        "gnome-terminal-",
                        "gnome-terminal",
                        "xterm",
                        "alacritty",
                        "kitty",
                        "wezterm-gui",
                        "konsole",
                        "lxterminal",
                        "xfce4-terminal",
                        "terminator",
                        "guake",
                        "tilda",
                    ];

                    for term in known_terms {
                        if name.contains(term) {
                            return name;
                        }
                    }

                    return name;
                }
            }
        }
    }

    "Unknown Terminal".to_string()
}

