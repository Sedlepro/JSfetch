mod info;

fn main() {
    let uptime = info::uptime::uptime();
    println!("Uptime: {} ", uptime);

}
