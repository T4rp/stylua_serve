use librojo::cli::BuildCommand;
use std::path::PathBuf;

fn main() {
    let command = BuildCommand {
        project: PathBuf::from("plugin"),
        output: Some(PathBuf::from("StyluaServePlugin.rbxm")),
        plugin: None,
        watch: false,
    };

    command.run().expect("Failed to build plugin");
}
