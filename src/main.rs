use clap::{Arg, SubCommand};
use cool_organizer::*;
use datetime::{LocalDate, Month};
use std::{fs, io::Write};
use std::io::{stdin,stdout};

fn main() {
    let default_path = TasksManager::default_path();

    let matches = clap::App::new("cool organizer")
        .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .default_value(&default_path)
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
        .subcommand(SubCommand::with_name("dadd")
                .about("adds a new task")
            )
    .get_matches()
    ;

    let path = matches.value_of("file").unwrap();
    let path = if path.ends_with(".toml") { path } else { &default_path };
    
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

        let _ = tasks.save(&default_path);
    }
    else if let Some(_arg) = matches.subcommand_matches("dadd") {
        // Do an "add task" dialog - use the stdout().flush() this time
        println!("Add task dialog init...");

        // get the task name
        let mut temp = String::new();

        print!("Name: ");
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut temp);
        let mut task = Task::new(temp.trim_end_matches('\n'));

        print!("Category: ");
        temp.clear();
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut temp);
        task.category = String::from(temp.trim_end_matches('\n'));

        print!("Sub category: ");
        temp.clear();
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut temp);
        task.sub_category = String::from(temp.trim_end_matches('\n'));

        print!("due(d/m/y): ");
        temp.clear();
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut temp);
        task.due = parse_to_date(temp.trim_end_matches('\n'));

        println!{"Task:"};
        println!("{}",task.formatted(true));

        tasks.add_task(task);

        let _ = tasks.save(path);

    }
    else {
        println!("{}", tasks.full_print_for_conky().trim_end_matches('\n'));
    }

}

fn parse_to_date(s : &str) -> Option<Date> {
    if s.is_empty() {
        None
    }
    else {
        let s = s.split('/').collect::<Vec<_>>();

        if s.len() == 3 {
            let d = match s[0].parse::<i16>() {
                Ok(d) => d,
                Err(_) => { return None; }
            };
            let m = match s[1].parse::<i16>() {
                Ok(m) => m,
                Err(_) => { return None; }
            };
            let y = match s[2].parse::<i64>() {
                Ok(y) => y,
                Err(_) => { return None; }
            };

            let m = match m {
                1 => Month::January,
                2 => Month::February,
                3 => Month::March,
                4 => Month::April,
                5 => Month::May,
                6 => Month::June,
                7 => Month::July,
                8 => Month::August,
                9 => Month::September,
                10 => Month::October,
                11 => Month::November,
                _ => Month::December,
            };

            let date = match LocalDate::ymd(y, m, d as i8) {
                Ok(d) => d,
                Err(_) => { return None; }
            };

            Some(Date::from(date))
        }
        else {
            None
        }
    }
}