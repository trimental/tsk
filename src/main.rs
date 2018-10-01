#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde_json;
extern crate termion;
extern crate xdg;

mod data;
mod display;
mod task;

use data::TskData;

use clap::{App, Arg, SubCommand};

fn main() {
    let mut tsk_data = TskData::new();

    // Clap
    let matches = App::new("tsk")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Manage tasks with tsk")
        .arg(
            Arg::with_name("completed")
                .short("c")
                .long("completed")
                .help("Show completed tasks when listing"),
        ).subcommand(
            SubCommand::with_name("new")
                .about("Create a new task")
                .alias("n")
                .alias("add")
                .arg(
                    Arg::with_name("title")
                        .index(1)
                        .takes_value(true)
                        .empty_values(false)
                        .required(true)
                        .help("Title of the task"),
                ).arg(
                    Arg::with_name("description")
                        .index(2)
                        .takes_value(true)
                        .empty_values(false)
                        .required(true)
                        .help("Description of the task"),
                ).arg(
                    Arg::with_name("priority")
                        .short("p")
                        .long("priority")
                        .takes_value(true)
                        .empty_values(false)
                        .possible_values(&["low", "medium", "high"])
                        .help("Priority of the task"),
                ),
        ).subcommand(
            SubCommand::with_name("del")
                .about("Delete a task")
                .alias("d")
                .arg(
                    Arg::with_name("id")
                        .index(1)
                        .takes_value(true)
                        .empty_values(false)
                        .validator(|id| {
                            if id.parse::<usize>().is_ok() {
                                Ok(())
                            } else {
                                Err("Argument is not a interger".into())
                            }
                        }).required_unless("all")
                        .help("id of the task"),
                ).arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Delete all tasks"),
                ),
        ).subcommand(
            SubCommand::with_name("info")
                .about("Display info about a task")
                .alias("i")
                .arg(
                    Arg::with_name("id")
                        .index(1)
                        .takes_value(true)
                        .empty_values(false)
                        .validator(|id| {
                            if id.parse::<usize>().is_ok() {
                                Ok(())
                            } else {
                                Err("Argument is not a interger".into())
                            }
                        }).required(true)
                        .help("id of the task"),
                ),
        ).subcommand(
            SubCommand::with_name("complete")
                .about("Create a new task")
                .alias("done")
                .alias("finish")
                .alias("c")
                .arg(
                    Arg::with_name("id")
                        .index(1)
                        .takes_value(true)
                        .empty_values(false)
                        .validator(|id| {
                            if id.parse::<usize>().is_ok() {
                                Ok(())
                            } else {
                                Err("Argument is not a interger".into())
                            }
                        }).required_unless("all")
                        .help("id of the task"),
                ).arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("Delete all tasks"),
                ),
        ).get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        // Create new task
        let title = matches.value_of("title").unwrap();
        let description = matches.value_of("description").unwrap();
        let mut task = task::Task::new(title, description);
        if let Some(priority) = matches.value_of("priority") {
            let priority = match priority {
                "low" => task::Priority::Low,
                "medium" => task::Priority::Medium,
                "high" => task::Priority::High,
                _ => unreachable!(),
            };
            task.priority = Some(priority);
        }
        tsk_data.add_task(task);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("del") {
        // Delete a task
        if matches.is_present("all") {
            tsk_data.delete_all();
        } else {
            let id = matches.value_of("id").unwrap().parse::<usize>().unwrap();
            tsk_data.delete_task(id);
        }
        return;
    }

    if let Some(matches) = matches.subcommand_matches("complete") {
        // Complete a task
        if matches.is_present("all") {
            tsk_data.complete_all();
        } else {
            let id = matches.value_of("id").unwrap().parse::<usize>().unwrap();
            tsk_data.complete_task(id);
        }
        return;
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        // Get info about a task
        let id = matches.value_of("id").unwrap().parse::<usize>().unwrap();
        let task = tsk_data.get_task(id).expect("Not a valid id");
        display::info(&task);
        return;
    }

    if matches.is_present("completed") {
        display::list_completed(&tsk_data.tasks);
        return;
    }

    // Display tasks
    display::list(&tsk_data.tasks, &tsk_data.order);
}
