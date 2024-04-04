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
            let list = task.next().unwrap();
            let task = task.next().unwrap();
            (list.to_string(), task.to_string())
        }
    )
    .collect()
}
 