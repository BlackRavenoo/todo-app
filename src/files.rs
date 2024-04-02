use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::PathBuf};

use crate::cli::select;

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
const TASK_NOT_FOUND: &str = "Task not found";

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
    .expect("Unable to deserialize json") //TODO match maybe
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
//TODO

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

//TODO: todo-app remove
//Если задание не найдено в указанном списке, то начать поиск по другим.
//Если в других списках найдено больше 1 подходящего задания,
//то вывести их в stdout и попросить пользователя указать номер.
pub fn remove_task(task: &str, list: &str) -> Result<(), String> {
    let mut data = get_file_data();

    if let Some(tasks) = data.get_mut(list) {
        if let Some(idx) = tasks.iter().position(|t| t == task) {
            let _ = tasks.remove(idx);
            save_file_data(&data);
            Ok(())
        } else {
            let mut tasks = Vec::new();
            for (other_list, tasks_) in data.iter() {
                if other_list != list {
                    if tasks_.iter().any(|t| t == task) {
                        tasks.push(other_list.to_string());
                    }
                }
            }

            match tasks.len() {
                0 => Err(TASK_NOT_FOUND.to_string()),
                1 => {
                    println!("The task was found in another list: {}", tasks[0]);
                    println!("Do you want to remove it? [y/n]");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Unable to read line");
                    if input.trim() == "y" {
                        data.entry(tasks[0].clone()).and_modify(|u| {
                            u.remove(
                                u.iter().position(|t| t == task).unwrap()
                            );
                        });
                        save_file_data(&data);
                        Ok(())
                    } else {
                        Err("".to_string())
                    }
                },
                _ => {
                    println!("The task was found in multiple lists:");
                    for (i, list) in tasks.iter().enumerate() {
                        println!("{}. {}", i + 1, list);
                    }
                    println!("Do you want to remove it from any of them? [y/n]");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Unable to read line");

                    if input.trim() != "y" {
                        return Err("".to_string());
                    }

                    tasks.push("exit".to_string());
                    let lists = select(tasks, Vec::from(["-m".to_string()])).unwrap();

                    if lists[0] == "exit" {
                        return Ok(());
                    }

                    for list in lists {
                        if list == "exit" {
                            continue;
                        }
                        data.entry(list.clone()).and_modify(|u| {
                            u.remove(
                                u.iter().position(|t| t == task).unwrap()
                            );
                        });
                    }

                    save_file_data(&data);

                    Ok(())
                }
            }


        }
    } else {
        Err(LIST_NOT_FOUND.to_string())
    }
}

//TODO: todo-app check
//Реализовать автодополнение. На вход идет "Сделать", а программа
//сперва ищет есть ли такое задание по основному листу, потом по другим.
//Если ничего не найдено, то ищет все задания начинающиеся так же
//(по-хорошему вообще реализовать поиск вхождений типа grep),
//Эти задания выводятся в чат и индексируются. Ждем ввода пользователя, 
//потом удаляем таску по индексу. Мейби стоит использовать что-то типа fzf
//или вообще сделать на выбор.
pub fn check_task(task: &str, list: Option<&str>) -> Result<bool, String> {
    let mut data = get_file_data();

    if let Some(list) = list {
        if let Some(tasks) = data.get_mut(list) {
            match tasks.iter_mut().find(|t| *t == task) {
                Some(task) => {
                    task.checked = !task.checked;
                    Ok(task.checked)
                }
                None => {
                    Err(TASK_NOT_FOUND.to_string())
                }
            }
        } else {
            Err(LIST_NOT_FOUND.to_string())
        }
    } else {
        for tasks in data.values_mut() {
            if let Some(task) = tasks.iter_mut().find(|t| *t == task) {
                task.checked = !task.checked;
                return Ok(task.checked);
            }
        };
        Err(TASK_NOT_FOUND.to_string())
    }
}


pub fn get_tasks(list: Option<&str>, with_list_name: bool) -> Result<Vec<Task>, String> {
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
            for (list, tasks_) in data.iter() {
                //TODO push list name 
                // if with_list_name {
                //     tasks.extend();
                // }
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
        file_dir.push("config.toml");
        std::fs::write(&file_dir, "default_list = \"default\"").expect("Unable to create file"); //TODO Реализовать трейт Default
                                                                                                                    //для конфига и записывать стандартный конфиг в файл
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