use crate::cli::Subcommands;
use crate::config::Settings;
use crate::files::{self, create_list, delete_list, get_lists, get_tasks};

use super::{parse_with_fzf, use_style};

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
        _ => {
            let tasks = parse_with_fzf();

            let len = tasks.len();
            
            for (list, task) in tasks {
                files::remove_task(&task, &list).unwrap();
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
    match (task, list) {
        (Some(task), None) => {
            match files::check_task(&task, &config.default_list) {
                Ok(true) => println!("{}", use_style("Task checked".to_string(), &config.output.text)),
                Ok(false) => println!("{}", use_style("Task unchecked".to_string(), &config.output.text)),
                Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
            }
        }
        (Some(task), Some(list)) => {
            match files::check_task(&task, &list) {
                Ok(true) => println!("{}", use_style("Task checked".to_string(), &config.output.text)),
                Ok(false) => println!("{}", use_style("Task unchecked".to_string(), &config.output.text)),
                Err(e) => eprintln!("{}", use_style(e, &config.output.err)),
            }
        }
        _ => {
            let tasks = parse_with_fzf();

            let len = tasks.len();
            
            for (list, task) in tasks {
                files::check_task(&task, &list).unwrap();
            }

            if len == 1 {
                println!("{}", use_style("Task checked/uncheked".to_string(), &config.output.text));
            } else {
                println!("{}", use_style("Tasks checked/uncheked".to_string(), &config.output.text))
            }
        }
   }
}

pub fn tasks(list: Option<String>, config: Settings) {
    match list {
        Some(list) => {
            match files::get_tasks(Some(&list)) {
                Ok(tasks) => {
                    tasks.iter().for_each(|task| println!("{}", use_style(task.to_string(), &config.output.text)));
                }
                Err(e) => {
                    eprintln!("{}", use_style(e, &config.output.err))
                }
            }
        }
        None => {
            get_lists().iter().for_each(|list| {
                let tasks = get_tasks(Some(list)).unwrap();
                if !tasks.is_empty() {
                    println!("{}", use_style(
                        format!("[{}]", list).to_string(),
                        &config.output.list
                    ));
                    tasks.iter().for_each(|task| {
                        println!("{}", use_style(task.to_string(), &config.output.text));
                    });
                }
            })
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
