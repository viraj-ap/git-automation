use std::process::{Command, exit};
use std::io::{self, Write};

fn main() {
    let check_branch_command = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to check current branch");

    let current_branch = String::from_utf8_lossy(&check_branch_command.stdout).trim().to_string();

    if current_branch != "main" {
        if current_branch == "HEAD" {
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

    update_commit_push();
}

fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo.");
        exit(1);
    }

    let commit_message = get_commit_message();

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

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
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
    io::stdout().flush().unwrap();

    let mut commit_message = String::new();
    io::stdin()
        .read_line(&mut commit_message)
        .expect("Failed to read line");

    commit_message.trim().to_string()
}
