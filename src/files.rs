use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::PathBuf};

use crate::cli::get_from_all_tasks;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub name: String,
    pub checked: bool,
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
pub const TASK_NOT_FOUND: &str = "Task not found";

fn get_file_data() -> HashMap<String, Vec<Task>> {
    let mut file_dir = get_dir();
    file_dir.push("tasks.json");
    if !&file_dir.exists() {
        let temp: HashMap<String, Vec<Task>> = HashMap::new();
        
        let file = File::create(&file_dir).expect("Unable to create file");
        serde_json::to_writer(file, &temp).expect("Unable to write to file");
    }

    serde_json::from_str(
        std::fs::read_to_string(&file_dir)
            .expect("Unable to read file")
            .as_str()
    )
    .expect("Unable to deserialize json")
}

fn save_file_data(data: &HashMap<String, Vec<Task>>) {
    let mut file_dir = get_dir();
    file_dir.push("tasks.json");
    let file = File::create(&file_dir).expect("Unable to create file");
    serde_json::to_writer(file, data).expect("Unable to write to file");
}

pub fn create_list(list: &str) -> Result<(), String> {
    let mut data = get_file_data();

    if data.contains_key(list) {
        return Err("List already exists".to_string());
    }

    data.insert(list.to_string(), Vec::new());

    save_file_data(&data);

    Ok(())
}

pub fn delete_list(list: &str) -> Result<(), String> {
    let mut data = get_file_data();

    if data.contains_key(list) {
        data.remove(list);
        save_file_data(&data);
        Ok(())
    } else {
        Err(LIST_NOT_FOUND.to_string())
    }
}

pub fn get_lists() -> Vec<String> {
    get_file_data().keys().map(|x| x.to_string()).collect()
}

pub fn add_task(task: &str, list: &str) -> Result<(), String> {
    let mut data = get_file_data();

    if let Some(tasks) = data.get_mut(list) {
        let task = Task {
            name: task.to_string(),
            checked: false,
        };
        if tasks.contains(&task) {
            return Err("Task already exists".to_string());
        }
        tasks.push(task);
        save_file_data(&data);
        Ok(())
    } else {
        Err(LIST_NOT_FOUND.to_string())
    }
}

pub fn remove_task(task: &str, list: &str) -> Result<(), String> {
    let mut data = get_file_data();

    if let Some(tasks) = data.get_mut(list) {
        if let Some(idx) = tasks.iter().position(|t| t == task) {
            let _ = tasks.remove(idx);
            save_file_data(&data);
            Ok(())
        } else {
            let lists = match get_from_all_tasks(&mut data, task, list, "check") {
                Ok(lists) => lists,
                Err(e) => return Err(e),
            };

            for list in lists {
                data.entry(list).and_modify(|u| {
                    u.remove(
                        u.iter().position(|t| t == task).unwrap()
                    );
                });
            }

            save_file_data(&data);

            Ok(())


        }
    } else {
        Err(LIST_NOT_FOUND.to_string())
    }
}

//TODO: todo-app check
//Если ничего не найдено, то ищет все задания начинающиеся так же
//(по-хорошему вообще реализовать поиск вхождений типа grep),
pub fn check_task(task: &str, list: &str) -> Result<bool, String> {
    let mut data = get_file_data();

    let status = if let Some(tasks) = data.get_mut(list) {
        match tasks.iter_mut().find(|t| *t == task) {
            Some(task) => {
                task.checked = !task.checked;
                Ok(task.checked)
            }
            None => {
                let lists = match get_from_all_tasks(&mut data, task, list, "check") {
                    Ok(lists) => lists,
                    Err(e) => return Err(e),
                };

                for list in lists {
                    data.entry(list).and_modify(|u| {
                        let task = u.iter_mut().find(|t| *t == task).unwrap();
                        task.checked = !task.checked;
                    });
                }

                save_file_data(&data);

                Ok(true)
            }
        }
    } else {
        Err(LIST_NOT_FOUND.to_string())
    };

    save_file_data(&data);

    status
}


pub fn get_tasks(list: Option<&str>) -> Result<Vec<Task>, String> {
    let data = get_file_data();

    match list {
        Some(list) => {
            if let Some(tasks) = data.get(list) {
                Ok(tasks.clone())
            } else {
                Err(LIST_NOT_FOUND.to_string())
            }
        }
        None => {
            let mut tasks: Vec<Task> = Vec::new();
            for (_, tasks_) in data.iter() {
                tasks.extend(tasks_.clone());
            }
            Ok(tasks)
        }
        
    }
}

pub fn check_dir() {
    let mut file_dir = get_dir();

    if !file_dir.exists() {
        std::fs::create_dir(&file_dir).expect("Unable to create directory");
    }

    file_dir.push("config.toml");

    if !file_dir.exists() {
        let config = crate::config::Settings::default();
        std::fs::write(
            &file_dir,
            toml::to_string(&config).expect("Failed to serialize config")
        ).expect("Unable to create file");
    }
    file_dir.pop();

    file_dir.push("tasks.json");

    if !file_dir.exists() {
        std::fs::write(file_dir, "{}").expect("Unable to create file");
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