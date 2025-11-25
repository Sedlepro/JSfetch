use std::fs;

pub fn get_cpu() -> String {

    if let Ok(contents) = fs::read_to_string("/proc/cpuinfo"){
        for line in contents.lines(){
            if line.starts_with("model name") || line.starts_with("Hardware"){
                return line.split(':')
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .to_string()
            }
        }
    }
"Unknown".to_string()

}
