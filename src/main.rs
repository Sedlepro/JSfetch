mod info;
mod ascii;

fn main() {
    println!("{}", ascii::get_ascii());

    let username = info::user::get_username();
    println!("User: {}", username);

    let uptime = info::uptime::uptime();
    println!("Uptime: {} ", uptime);

    let cpu = info::cpu::get_cpu();
    println!("CPU: {}", cpu);

    let gpu = info::gpu::get_gpu();
    println!("GPU : {}", gpu);

    let mem = info::memory::get_mem();
    println!("Used RAM: {} GB | Total RAM: {} GB", mem.used_gb, mem.total_gb);
    
    let terminal = info::terminal::terminal();
    println!("Terminal: {}", terminal);

    let os = info::system::get_os();
    println!("Linux Distro: {}", os);
    let shell = info::shell::get_shell();
    println!("Shell : {}", shell);


    let (percent, status) = info::battery::battery();
    println!("Battery: {}% ({})", percent, status);
}
