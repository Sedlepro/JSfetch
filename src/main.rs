mod info;

fn main() {
    let username = info::user::get_username();
    println!("User: {}", username);

    let uptime = info::uptime::uptime();
    println!("Uptime: {} ", uptime);

    let cpu = info::cpu::get_cpu();
    println!("CPU: {}", cpu);
    
    let term = info::terminal::terminal();
    println!("Terminal: {}", term);
}
