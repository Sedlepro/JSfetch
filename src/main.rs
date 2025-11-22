mod info;

fn main() {
    let username = info::user::get_username();
    println!("User: {}", username);

    let uptime = info::uptime::uptime();
    println!("Uptime: {} ", uptime);
    
}
