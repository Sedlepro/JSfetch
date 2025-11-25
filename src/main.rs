mod info;

fn main() {
    let uptime = info::uptime::uptime();
    println!("Uptime: {} ", uptime);
    let term = info::terminal::terminal();
    println!("Terminal: {}", term);
}
