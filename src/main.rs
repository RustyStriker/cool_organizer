use clap::{Arg, SubCommand};
use cool_organizer::*;
use std::fs;

fn main() {
    const DEFAULT_FILE : &str = "./tasks.toml";


    let matches = clap::App::new("cool organizer")
        .arg(Arg::with_name("file")
                .short("t")
                .long("task")
                .default_value(DEFAULT_FILE)
                .takes_value(true)
                .value_name("FILE")
            )
        .arg(Arg::with_name("conky")
                .help("prints normal output for conky")
                .short("c")
                .long("conky")
                .takes_value(false)    
            )
        .subcommand(SubCommand::with_name("create_example")
                .about("creates a default config")
            )
    .get_matches()
    ;

    let path = matches.value_of("file").unwrap();
    let path = if path.ends_with(".toml") { path } else { DEFAULT_FILE };
    
    let file = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => String::new(),
    };

    let mut tasks : TasksManager = toml::from_str(&file).unwrap_or(TasksManager::default());

    if let Some(_arg) = matches.subcommand_matches("create_example") {
        tasks.tasks.push(
            Task::new("example")
                .category("examplish")
                .sub_category("sub_category")
                .due(Some(Date::ymd(2021, 79)))
        );

        let _ = tasks.save(DEFAULT_FILE);
    }
    else {
        println!("{}", tasks.full_print_for_conky());
    }

}
