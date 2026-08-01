use std::error::Error;
use std::process::Command;

pub fn run_docker_container(name: &String) -> Result<(), Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("docker run {name}")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;

    println!("{name}");
    Ok(())
}
