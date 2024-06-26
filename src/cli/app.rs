use std::{error::Error, process::exit};

use clap::{arg, command, ArgMatches, Command};

#[derive(Debug)]
pub enum Subcommands {
    Add(Option<String>, Option<String>),
    Remove(Option<String>, Option<String>),
    Check(Option<String>, Option<String>),
    Tasks(Option<String>),
    AddList(Option<String>),
    RemoveList(Option<String>),
    Lists,
}

#[derive(Debug)]
pub struct Config {
    pub subcommand: Option<Subcommands>,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    //TODO: read config from file(default list)
    let command = command!()
        .arg_required_else_help(true) //TODO: delete
        .subcommand(
            Command::new("add")
                .arg(arg!(task_name: <TASK> "Task name"))
                .arg(
                    arg!(list_name: <LIST> "List name")
                        .required(false)
                )
                .about("Add a new task"),
        )
        .subcommand(
            Command::new("remove")
                .arg(
                    arg!(task_name: <TASK> "Task name")
                        .required(false)
                )
                .arg(
                    arg!(list_name: <LIST> "List name")
                        .required(false)
                )
                .about("Remove a task"),
        )
        .subcommand(
            Command::new("check")
                .arg(arg!(task_name: <TASK> "Task name").required(false))
                .arg(
                    arg!(list_name: <LIST> "List name")
                        .required(false)
                )
                .about("Check/uncheck task"),
        )
        .subcommand(
            Command::new("tasks")
                .arg(arg!(list_name: <LIST> "List name").required(false))
                .about("Print all tasks"),
        )
        .subcommand(
            Command::new("add-list")
                .arg(arg!(list_name: <LIST> "List name"))
                .about("Create a new list"),
        )
        .subcommand(
            Command::new("remove-list")
                .arg(arg!(list_name: <LIST> "List name"))
                .about("Delete the list"),
        )
        .subcommand(Command::new("lists").about("Print all lists"));

    let matches = command.get_matches();
    
    let subcommand = matches.subcommand();

    match subcommand {
        Some((subcmd, args)) => match subcmd {
            "add" => Ok(Config {
                subcommand: Some(Subcommands::Add(
                    get_string("task_name", args),
                    get_string("list_name", args),
                )),
            }),
            "remove" => Ok(Config {
                subcommand: Some(Subcommands::Remove(
                    get_string("task_name", args),
                    get_string("list_name", args),
                )),
            }),
            "check" => Ok(Config {
                subcommand: Some(Subcommands::Check(
                    get_string("task_name", args),
                    get_string("list_name", args),
                )),
            }),
            "tasks" => Ok(Config {
                subcommand: Some(Subcommands::Tasks(get_string("list_name", args))),
            }),
            "add-list" => Ok(Config {
                subcommand: Some(Subcommands::AddList(get_string("list_name", args))),
            }),
            "remove-list" => Ok(Config {
                subcommand: Some(Subcommands::RemoveList(get_string("list_name", args))),
            }),
            "lists" => Ok(Config {
                subcommand: Some(Subcommands::Lists),
            }),
            _ => Err("Wrong subcommand".into()),
        },
        None => Ok(Config { subcommand: None }),
    }
}

fn get_string(id: &str, args: &ArgMatches) -> Option<String> {
    match args.get_one::<String>(id) {
        Some(list) => match list.as_str() {
            "" => {
                eprintln!("The {} can't be an empty string", id);
                exit(1)
            }
            _ => Some(list.into()),
        },
        None => None,
    }
}
