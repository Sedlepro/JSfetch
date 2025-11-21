// love u sed <3 
use std::process::Command;
fn main()
{
    let OS = get_os();
    let UPTIME = get_uptime();
    let SHELL = get_shell();
    let KERNEL = get_kernel();

    println!("==WELCOME TO JSFETCH==");
    println!("--Brought to you by Jessica and Sedrick--");
    println!("Current OS: {}", OS);
    println!("Uptime: {}", UPTIME);
    println!("Shell: {}", SHELL);
    println!("Kernel: {}", KERNEL);

    fn get_os() -> String {
        "Unknown_OS".to_string()
    }
    fn get_uptime() -> String {
        "Unknown_Uptime".to_string()
    }
    fn get_shell() -> String {
        "Unknown_Shell".to_string()
    }
    fn get_kernel() -> String {
        "Unknown_Kernel".to_string()
    }

    //println!("love u sed gros bisous");
    //bruh
    //todo list
    //- faire le ascii
    //- faire les infos
}
