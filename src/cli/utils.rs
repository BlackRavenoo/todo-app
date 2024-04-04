use std::{collections::HashMap, process::exit};

use crate::files::{get_lists, get_tasks, Task};

use super::select;

pub fn parse_with_fzf() -> Vec<(String, String)> {
    let mut lists = get_lists();
    lists.push("All".to_string());

    lists = lists.into_iter().filter(|list| !list.is_empty()).collect();

    let list = &select(lists.clone(), Vec::new()).unwrap()[0];

    let tasks = if list == "All" {
        lists.pop();
        lists.into_iter().map(
            |list| {
                get_tasks(Some(&list)).unwrap().into_iter().map(
                    |mut task| {
                        task.name = format!("{}: {}", list, task.name);
                        task
                    }
                ).collect::<Vec<Task>>()
            
            }
        ).flatten().collect()
    } else {
        get_tasks(Some(list)).unwrap()
    };

    let tasks = select (
        tasks
        .into_iter()
        .map(|task| task.name)
        .collect(),
        vec!["-m".to_string()]
    )
    .unwrap();

    tasks.into_iter().map(
        |task| {
            let mut task = task.split(": ");
            let list = if list == "All" {task.next().unwrap()} else {list};
            let task = task.next().unwrap();
            (list.to_string(), task.to_string())
        }
    )
    .collect()
}

pub fn get_from_all_tasks(data: &mut HashMap<String, Vec<Task>>, task: &str, list: &str, action: &str) -> Result<Vec<String>, String> {
    let mut tasks = Vec::new();
    for (other_list, tasks_) in data.iter() {
        if other_list != list {
            if tasks_.iter().any(|t| t == task) {
                tasks.push(other_list.to_string());
            }
        }
    }
    match tasks.len() {
        0 => Err(crate::files::TASK_NOT_FOUND.to_string()),
        1 => {
            println!("The task was found in another list: {}", &tasks[0]);
            println!("Do you want to {} it? [y/N]", action);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Unable to read line");
            if input.trim().to_ascii_lowercase() == "y" {
                Ok(vec![tasks[0].clone()])
            } else {
                Err("".to_string())
            }
        },
        _ => {
            println!("The task was found in multiple lists:");
            for (i, list) in tasks.iter().enumerate() {
                println!("{}. {}", i + 1, list);
            }
            println!("Do you want to {} it in any of them? [y/N]", action);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Unable to read line");

            if input.trim().to_ascii_lowercase() == "y" {
                
                let lists = select(tasks, Vec::from(["-m".to_string()])).unwrap();
    
                Ok(lists)
            } else {
                exit(0);
            }
        }
    }
}