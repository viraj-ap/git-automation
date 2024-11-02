use std::process::{Command, exit};
use std::io::{self, Write};

fn update_commit_push() {
    // Command 1: Add all files recursively to git repo
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo.");
        exit(1);
    }

    // Get commit message from user
    let commit_message = get_commit_message();

    // Command 2: Commit all changes with user-provided message
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes.");
        exit(1);
    }

    // Command 3: Push to remote (origin main)
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main") // Change from "master" to "main"
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes to remote.");
        exit(1);
    }

    println!("Successfully added, committed, and pushed changes!");
}

fn get_commit_message() -> String {
    print!("Enter commit message: ");
    io::stdout().flush().unwrap(); // Ensure prompt is printed

    let mut commit_message = String::new();
    io::stdin()
        .read_line(&mut commit_message)
        .expect("Failed to read line");

    commit_message.trim().to_string() // Trim whitespace and return
}

fn main() {
    // Check if we are in a Git repository
    let check_git_command = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output();

    if let Err(_) = check_git_command {
        eprintln!("Error: Not in a Git repository. Please initialize a repository first.");
        exit(1);
    }

    // Ensure the branch is named main
    let check_branch_command = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to check current branch");

    let current_branch = String::from_utf8_lossy(&check_branch_command.stdout).trim().to_string();

    if current_branch != "main" {
        println!("Warning: Current branch is '{}' instead of 'main'.", current_branch);
        println!("Please switch to 'main' or create it if it doesn't exist.");
        exit(1);
    }

    // Optional: Check if remote 'origin' exists, and prompt for URL if not
    let remote_check_command = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("Failed to check for remotes");

    let remote_output = String::from_utf8_lossy(&remote_check_command.stdout);

    if !remote_output.contains("origin") {
        println!("No remote 'origin' found.");
        print!("Please enter the remote repository URL: ");
        io::stdout().flush().unwrap(); // Ensure prompt is printed

        let mut remote_url = String::new();
        io::stdin()
            .read_line(&mut remote_url)
            .expect("Failed to read line");
        
        let add_remote_command = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(remote_url.trim())
            .output()
            .expect("Failed to add remote");

        if !add_remote_command.status.success() {
            eprintln!("Error: Failed to add remote origin.");
            exit(1);
        }
    }

    update_commit_push();
}
