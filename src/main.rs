use std::process::{Command, exit};
use std::io::{self, Write}; // For user input

fn main() {
    // Ensure the branch is named main
    let check_branch_command = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to check current branch");

    let current_branch = String::from_utf8_lossy(&check_branch_command.stdout).trim().to_string();

    // If the current branch is not 'main', create it
    if current_branch != "main" {
        if current_branch == "HEAD" {
            // If in detached HEAD state, create the main branch
            let create_branch_command = Command::new("git")
                .arg("checkout")
                .arg("-b")
                .arg("main")
                .output()
                .expect("Failed to create main branch");

            if !create_branch_command.status.success() {
                eprintln!("Error: Failed to create 'main' branch.");
                exit(1);
            }
        } else {
            println!("Warning: Current branch is '{}' instead of 'main'.", current_branch);
            println!("Please switch to 'main' or create it if it doesn't exist.");
            exit(1);
        }
    }

    // Proceed to update, commit, and push changes
    update_commit_push();
}

fn update_commit_push() {
    // Command 1: Add all files recursively to git repo
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
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
        eprintln!("Error: Failed to commit changes. Check if there are any changes to commit.");
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
        eprintln!("Error: Failed to push changes to remote. Ensure you have a remote origin set.");
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
