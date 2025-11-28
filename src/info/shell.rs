use std::fs;
use std::env;

pub fn get_shell() -> String {
    // 1. Le shell ACTUEL (proc/self/comm)
    if let Ok(comm) = fs::read_to_string("/proc/self/comm") {
        let name = comm.trim().to_string();

        // filtrer les shells connus
        let known_shells = ["bash", "zsh", "fish", "sh", "dash"];

        for s in known_shells {
            if name.contains(s) {
                return s.to_string();
            }
        }
    }

    // 2. Fallback: SHELL par défaut (variable d’environnement)
    if let Ok(shell_path) = env::var("SHELL") {
        // extraire juste le nom du binaire (ex: /bin/bash → bash)
        if let Some(name) = shell_path.split('/').last() {
            return name.to_string();
        }
        return shell_path;
    }

    // 3. Sinon inconnu
    "Unknown Shell".to_string()
}

