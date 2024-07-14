use std::process::Command;

fn main() {
    Command::new("rojo")
        .args(["build", "plugin", "-o", "StyluaServePlugin.rbxm"])
        .spawn()
        .unwrap();
}