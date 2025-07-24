use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// A helper function to get a Command instance for our binary
fn get_cmd() -> Command {
    Command::cargo_bin("todo").unwrap()
}

#[test]
fn test_add_and_list() {
    let test_file = "test_add_and_list.json";
    // Ensure the file is clean before the test
    let _ = fs::remove_file(test_file);

    // 1. Add a task
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "add", "Buy milk"])
        .assert()
        .success();

    // 2. List the tasks and verify the new task is there
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Index"))
        .stdout(predicate::str::contains("Description"))
        .stdout(predicate::str::contains("Due Date"))
        .stdout(predicate::str::contains("Completed"))
        .stdout(predicate::str::contains("Buy milk"));

    // Clean up the test file
    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_complete_task() {
    let test_file = "test_complete_task.json";
    // Ensure the file is clean before the test
    let _ = fs::remove_file(test_file);

    // 1. Add a task
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "add", "Wash the car"])
        .assert()
        .success();

    // 2. Complete the task
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "complete", "0"])
        .assert()
        .success();

    // 3. List the tasks and verify the task is marked as completed
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Index"))
        .stdout(predicate::str::contains("Description"))
        .stdout(predicate::str::contains("Due Date"))
        .stdout(predicate::str::contains("Completed"))
        .stdout(predicate::str::contains("Yes"));

    // Clean up the test file
    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_remove_task() {
    let test_file = "test_remove_task.json";
    // Ensure the file is clean before the test
    let _ = fs::remove_file(test_file);

    // 1. Add a task
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "add", "Mow the lawn"])
        .assert()
        .success();

    // 2. Remove the task
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "remove", "0"])
        .assert()
        .success();

    // 3. List the tasks and verify the task is gone
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks to display."));

    // Clean up the test file
    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_invalid_index() {
    let test_file = "test_invalid_index.json";
    // Ensure the file is clean before the test
    let _ = fs::remove_file(test_file);

    // Try to complete a task that doesn't exist
    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "complete", "99"])
        .assert()
        .success() // The program should still exit successfully
        .stdout(predicate::str::contains("No task at index 99"));
}

#[test]
fn test_cleanup() {
    let test_file = "test_cleanup.json";

    let _ = fs::remove_file(test_file);

    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "add", "Fix the bugs"])
        .assert()
        .success();

    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "complete", "0"])
        .assert()
        .success();

    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "clean"]).assert().success();

    let mut cmd = get_cmd();
    cmd.args(["--file", test_file, "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No tasks to display."));

    fs::remove_file(test_file).unwrap();
}
