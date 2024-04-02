use crate::cli::Subcommands;
use crate::config::Settings;
use crate::files::{self, create_list, delete_list, get_lists, get_tasks};

use super::{select, use_style};

pub fn parse_args(subcmd: Subcommands, config: Settings) {
    match subcmd {
        Subcommands::Add(task, list) => add(task, list, config),
        Subcommands::Remove(task, list) => remove(task, list, config),
        Subcommands::Check(task, list) => check(task, list, config),
        Subcommands::Tasks(list) => tasks(list, config),
        Subcommands::AddList(list) => add_list(list, config),
        Subcommands::RemoveList(list) => remove_list(list, config),
        Subcommands::Lists => lists(config),
    }
}

pub fn add(task: Option<String>, list: Option<String>, config: Settings) {
    match (task, list) {
        (Some(task), None) => {
            match files::add_task(&task, &config.default_list) {
                Ok(_) => {
                    println!("{}", use_style("Task added".to_string(), &config.output.text));
                }
                Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
            };
        }
        (Some(task), Some(list)) => {
            match files::add_task(&task, &list) {
                Ok(_) => {
                    println!("{}", use_style("Task added".to_string(), &config.output.text));
                }
                Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
            };
        }
        _ => unreachable!()
    }
}

pub fn remove(task: Option<String>, list: Option<String>, config: Settings) {
   match (task, list) {
        (Some(task), None) => {
            match files::remove_task(&task, &config.default_list) {
                Ok(_) => println!("{}", use_style("Task deleted".to_string(), &config.output.text)),
                Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
            }
        }
        (Some(task), Some(list)) => {
            match files::remove_task(&task, &list) {
                Ok(_) => println!("{}", use_style("Task deleted".to_string(), &config.output.text)),
                Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
            }
        }
        //TODO Проверять пустой ли список и если пустой, то не выводить его
        _ => {
            let mut lists = get_lists();
            lists.push("All".to_string());

            let list = &select(lists, Vec::new()).unwrap()[0];

            let list = if list == "All" {
                unimplemented!() //TODO
            } else {
                list
            };

            let tasks = get_tasks(Some(list), false).unwrap();

            let tasks = select (
                tasks
                .into_iter()
                .map(|task| task.name)
                .collect(),
                vec!["-m".to_string()]
            )
            .unwrap();

            let len = tasks.len();
            
            for task in tasks {
                files::remove_task(&task, list).unwrap();
            }

            if len == 1 {
                println!("{}", use_style("Task deleted".to_string(), &config.output.text));
            } else {
                println!("{}", use_style("Tasks deleted".to_string(), &config.output.text))
            }
        }
   }
}

pub fn check(task: Option<String>, list: Option<String>, config: Settings) {
    match task {
        Some(task) => {
            match files::check_task(&task, list.as_deref()) {
                Ok(true) => println!("{}", use_style("Task checked".to_string(), &config.output.text)),
                Ok(false) => println!("{}", use_style("Task unchecked".to_string(), &config.output.text)),
                Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
            }
        }
        None => unreachable!()
   }
}

pub fn tasks(list: Option<String>, config: Settings) {
    match files::get_tasks(list.as_deref(), true) {
        Ok(tasks) => {
            tasks.iter().for_each(|task| println!("{}", use_style(task.to_string(), &config.output.text)));
        }
        Err(e) => {
            eprintln!("{}", use_style(e, &config.output.err))
        }
    }
}

pub fn add_list(list: Option<String>, config: Settings) {
    match create_list(&list.unwrap()) {
        Ok(_) => println!("{}", use_style("List added".to_string(), &config.output.text)),
        Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
    }
}

pub fn remove_list(list: Option<String>, config: Settings) {
    match delete_list(&list.unwrap()) {
        Ok(_) => println!("{}", use_style("List removed".to_string(), &config.output.text)),
        Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
    }
}

pub fn lists(config: Settings) {
    get_lists().iter().for_each(|task| println!("{}", use_style(task.to_string(), &config.output.text)));
}
