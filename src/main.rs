use std::process::Command;

fn main() {
    Command::new("rustup")
        .arg("self")
        .arg("uninstall")
        .arg("-y")
        .status()
        .expect("Failed to uninstall Rust");
}
