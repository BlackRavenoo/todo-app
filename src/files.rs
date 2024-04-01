use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    checked: bool,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

impl PartialEq<str> for Task {
    fn eq(&self, other: &str) -> bool {
        self.name == other.to_string()
    }
    fn ne(&self, other: &str) -> bool {
        self.name != other.to_string()
    }
}

impl Into<clap::builder::Str> for Task {
    fn into(self) -> clap::builder::Str {
        clap::builder::Str::from(self.name)
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            if self.checked {"☑"} else {"x"},
            self.name
        )
    }
}

const LIST_NOT_FOUND: &str = "List not found";
const TASK_NOT_FOUND: &str = "Task not found";

pub fn create_list(list: &str) -> Result<(), String> {
    let mut file_dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("/tmp"),
    };
    file_dir.push(".todo-app");
    file_dir.push(format!("{}.jsonl", list));
    match std::fs::write(file_dir, "") {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn remove_list(list: &str) -> Result<(), String> {
    let mut file_dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("/tmp"),
    };
    file_dir.push(".todo-app");
    file_dir.push(format!("{}.jsonl", list));
    if file_dir.exists() {
        std::fs::remove_file(file_dir).expect("Unable to remove file");
        Ok(())
    } else {
        Err("List does not exist".to_string())
    }
}

pub fn get_lists() -> Vec<String> {
    let mut file_dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("/tmp"),
    };
    file_dir.push(".todo-app");
    let mut lists = Vec::new();
    for entry in std::fs::read_dir(file_dir).expect("Unable to read directory") {
        let entry = entry.expect("Unable to read entry");
        let path = entry.path();
        if path.extension().and_then(|x| x.to_str()) == Some("jsonl") {
            lists.push(
                path.file_stem()
                    .expect("Unable to get file stem")
                    .to_str()
                    .expect("Unable to convert to str")
                    .to_string(),
            );
        }
    }
    lists
}
//TODO

pub fn add_task(task: &str, list: &str) -> Result<(), String> {
    let mut file_dir = get_dir();
    file_dir.push(format!("{}.jsonl", list));

    if file_dir.exists() {
        let file = &std::fs::read_to_string(&file_dir).expect("Unable to read file");
        let mut tasks: Vec<Task> = if !file.is_empty() {
            serde_json::from_str(file).expect("Unable to deserialize json")
        } else {
            return Err("Unable to deserialize json".to_string());
        };
        let task = Task {
            name: task.to_string(),
            checked: false,
        };
        if tasks.contains(&task) {
            return Err("Task already exists".to_string());
        }
        tasks.push(task);
        std::fs::write(
            file_dir,
            serde_json::to_string(&tasks).expect("Unable to serialize json"),
        )
        .expect("Unable to write to file");
        Ok(())
    } else {
        Err(LIST_NOT_FOUND.to_string())
    }
}

//TODO: todo-app remove
//Если задание не найдено в указанном списке, то начать поиск по другим.
//Если в других списках найдено больше 1 подходящего задания,
//то вывести их в stdout и попросить пользователя указать номер.
pub fn remove_task(task: &str, list: &str) -> Result<(), String> {
    let mut file_dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("/tmp")
    };
    
    file_dir.push(".todo-app");
    file_dir.push(format!("{}.jsonl", list));

    if file_dir.exists() {
        let mut tasks: Vec<Task> = serde_json::from_str(
                &std::fs::read_to_string(&file_dir)
                .expect("Unable to read file")
            )
            .expect("Unable to deserialize json");

        if let Some(idx) = tasks.iter().position(|t| t == task) {
            let _ = tasks.remove(idx);
            std::fs::write(file_dir, serde_json::to_string(&tasks).expect("Unable to serialize json"))
                .expect("Unable to write to file");
            Ok(())
        } else {
            Err(TASK_NOT_FOUND.to_string())
        }
    } else {
        Err(LIST_NOT_FOUND.to_string())
    }
}

//TODO: todo-app check
//Если задание не найдено в указанном списке, то начать поиск по другим.
//Если в других списках найдено больше 1 подходящего задания,
//то вывести их в stdout и попросить пользователя указать номер.
//TODO: todo-app check
//Реализовать автодополнение. На вход идет "Сделать", а программа
//сперва ищет есть ли такое задание по основному листу, потом по другим.
//Если ничего не найдено, то ищет все задания начинающиеся так же
//(по-хорошему вообще реализовать поиск вхождений типа grep),
//Эти задания выводятся в чат и индексируются. Ждем ввода пользователя, 
//потом удаляем таску по индексу. Мейби стоит использовать что-то типа fzf
//или вообще сделать на выбор.
pub fn check_task(task: &str, list: Option<&str>) -> Result<bool, String> {
    let mut file_dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("/tmp")
    };
    file_dir.push(".todo-app");
    match list {
        Some(list) => {
            file_dir.push(format!("{}.jsonl", list));
            if !file_dir.exists() {
                return Err(LIST_NOT_FOUND.to_string());
            }
            let mut tasks: Vec<Task> = serde_json::from_str(&std::fs::read_to_string(&file_dir).unwrap()).unwrap();
            match tasks.iter_mut().find(|task_| task_.name == task) {
                Some(task) => {
                    task.checked = !task.checked;
                    let status = task.checked;
                    std::fs::write(file_dir, serde_json::to_string(&tasks).unwrap()).unwrap();
                    Ok(status)
                },
                None => { //TODO #1
                    for entry in std::fs::read_dir(file_dir).expect("Unable to read directory") {
                        let entry = entry.expect("Unable to read entry");
                        let path = entry.path();
                        if path.extension().and_then(|x| x.to_str()) == Some("jsonl") { //TODO: Заменить это на отдельную функцию, ибо часто используется везде
                            let tasks_list: Vec<Task> = serde_json::from_str(
                                &std::fs::read_to_string(path).expect("Unable to read file"),
                            )
                            .expect("Unable to deserialize json");
                            tasks.extend(tasks_list); //TODO
                        }
                    }
                    Err(TASK_NOT_FOUND.to_string()) 
                }
            }
        },
        None => {
            todo!("Not implemented yet")
            //TODO
        },
    }
}


pub fn get_tasks(list: Option<&str>) -> Result<Vec<Task>, String> {
    let mut file_dir = get_dir();
    match list {
        Some(list) => {
            file_dir.push(format!("{}.jsonl", list));
            if file_dir.exists(){
                Ok(serde_json::from_str(&std::fs::read_to_string(file_dir).expect("Unable to read file"))
                    .expect("Unable to deserialize json"))
            } else {
                Err(LIST_NOT_FOUND.to_string())
            }
        }
        None => {
            let mut tasks: Vec<Task> = Vec::new();
            for entry in std::fs::read_dir(file_dir).expect("Unable to read directory") {
                let entry = entry.expect("Unable to read entry");
                let path = entry.path();
                if path.extension().and_then(|x| x.to_str()) == Some("jsonl") {
                    let tasks_list: Vec<Task> = serde_json::from_str(
                            &std::fs::read_to_string(path).expect("Unable to read file"),
                        )
                        .expect("Unable to deserialize json");
                    tasks.extend(tasks_list);
                }
            }
            Ok(tasks)
        }
    }
}

pub fn check_dir() {
    let mut file_dir = get_dir();

    if !file_dir.exists() {
        std::fs::create_dir(&file_dir).expect("Unable to create directory");
        file_dir.push("config.toml");
        std::fs::write(&file_dir, "default_list = \"default\"").expect("Unable to create file");
        file_dir.pop();
        file_dir.push("default.jsonl");
        std::fs::write(file_dir, "").expect("Unable to create file");
    }
}

fn get_dir() -> PathBuf {
    let mut file_dir = match dirs::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("/tmp"),
    };
    file_dir.push(".todo-app");
    file_dir
}