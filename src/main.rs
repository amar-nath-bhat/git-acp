use std::process::{exit, Command};

fn commit_msg() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a commit message!");
        exit(1);
    }

    args[1..].join(" ")
}

fn run_command(command: &mut Command) -> String {
    let output = command.output().expect("Failed to execute command!");

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn add_commit_push() {
    // Add files
    println!("Adding files...");
    run_command(Command::new("git").arg("add").arg("-A"));
    println!("Files added successfully!");

    // Commit changes
    println!("Committing changes...");
    run_command(
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_msg()),
    );
    println!("Files committed successfully!");

    // Push changes
    println!("Pushing to remote repository...");
    run_command(Command::new("git").arg("push").arg("origin").arg("main"));
    println!("Files pushed successfully!");
}

fn main() {
    add_commit_push();
}
