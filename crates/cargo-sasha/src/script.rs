use std::process::Command;

pub fn run_script(path: &str) {
    let status = Command::new(path)
        .status()
        .expect("Failed to run script");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}



