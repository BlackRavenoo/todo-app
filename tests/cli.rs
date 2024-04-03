use assert_cmd::Command;
use predicates::prelude::*;

//Works only with test-threads = 1
//TODO: Add functionality to create separate files for each test

fn create_test_list() -> String {
    let mut cmd = Command::cargo_bin("todo-app").unwrap();
    let lists = cmd.arg("lists").output().unwrap();
    let mut lists = std::str::from_utf8(&lists.stdout).unwrap().split("\n").collect::<Vec<_>>();
    lists.pop();
    let mut name = uuid::Uuid::new_v4().to_string();
    while lists.contains(&name.as_str()) {
        name = uuid::Uuid::new_v4().to_string();
    }
    let mut cmd = Command::cargo_bin("todo-app").unwrap();
    cmd.args(&["add-list", &name]).assert().success();
    name
}

fn delete_test_list(list: &str) {
    let mut cmd = Command::cargo_bin("todo-app").unwrap();
    cmd.args(&["remove-list", list]).assert().success();
}

fn add_some_task(list: &str) -> assert_cmd::assert::Assert{
    let mut cmd = Command::cargo_bin("todo-app").unwrap();
    cmd.args(&["add", "some_task", list]).assert()
}

#[test]
fn add_tasks_works() {
    let list = create_test_list();
    
    let assert = add_some_task(list.as_str());

    delete_test_list(&list);

    assert.success().stdout("Task added\n");
}

#[test]
fn tasks_list_works() {
    let mut cmd = Command::cargo_bin("todo-app").unwrap();
    
    let list = create_test_list();

    add_some_task(list.as_str());

    let assert = cmd.arg("tasks").assert();

    delete_test_list(&list);

    assert.success().stdout(predicate::str::contains("some_task"));
}