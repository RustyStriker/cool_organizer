use clap::{Arg, SubCommand};
use cool_organizer::*;
use datetime::{DatePiece, LocalDate, Month};
use std::{fs, io::{Read, Write}};
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
        .arg(Arg::with_name("remove_done")
                .help("removes all past tasks that are done")
                .short("r")
                .long("remove_done")
                .takes_value(false)
            )
        .subcommand(SubCommand::with_name("create_example")
                .about("creates a default config")
            )
        .subcommand(SubCommand::with_name("add")
                .about("adds a new task")
            )
        .subcommand(SubCommand::with_name("edit")
                .about("edit a task")
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

    let mut should_save = false;

    // Argument matches
    if let Some(_a) = matches.args.get("remove_done") {
        tasks.remove_done();
        should_save = true;
    }

    // sub command matches
    let (command, _command_args) = matches.subcommand();
    match command {
        "create_example" => {
            tasks.tasks.push(
                Task::new("example")
                    .category("examplish")
                    .sub_category("sub_category")
                    .due(Some(Date::ymd(2021, 79)))
            );
    
            should_save = true;
        }
        "add" => {
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
    
            should_save = true;
    
        }
        "edit" => {
            should_save = true;

            loop { // Choose task loop
                println!("Tasks:");
                println!("{}",tasks.tasks_list());
                print!("Task to edit(-1 to quit): ");
                let _ = stdout().flush();
                let mut task_number = String::with_capacity(4);
                let _ = stdin().read_line(&mut task_number);
                let t : i32 = match task_number.trim_end_matches('\n').parse() {
                    Ok(i) => { i }
                    Err(e) => { 
                        println!("ERROR: {:?}",e);
                        continue;
                    }
                };
                // got a task, what do we want to change?
                // lets make sure we have a valid number tho
                if t == -1 {
                    break;
                } 
                else if t < -1 || t >= tasks.tasks.len() as i32 {
                    println!("invalid number");
                    continue;
                }

                loop {
                    let mut t = tasks.tasks.get_mut(t as usize).unwrap();

                    // Edit loop
                    println!("Editing task:");
                    println!("{}",t.formatted(true));
                    println!("Choose a property to edit:");
                    println!("0 - name\n1 - category\n2 - sub category");
                    println!("3 - priority\n4 - due date\n5 - done status");
                    println!("(-1/s) - go back to task selection");
                    let _ = stdout().flush();

                    let mut property = String::with_capacity(3);
                    let _ = stdin().read_line(&mut property);
                    let mut buff = String::new();
                    match property.trim_end_matches('\n') {
                        "-1" | "s" => break,
                        "0" => {
                            println!("current: {}", &t.name);
                            print!("new: ");
                            stdout().flush().expect("couldnt flush output");
                            stdin().read_line(&mut buff).expect("coudlnt get input");
                            if buff.trim_matches('\n').is_empty() {
                                println!("task name cannot be empty!");
                                continue;
                            }
                            t.name = String::from(buff.trim_matches('\n'));
                        }
                        "1" => {
                            println!("current: {}", &t.category);
                            print!("new: ");
                            stdout().flush().expect("couldnt flush output");
                            stdin().read_line(&mut buff).expect("coudlnt get input");
                            t.category = String::from(buff.trim_matches('\n'));
                        }
                        "2" => {
                            println!("current: {}", &t.sub_category);
                            print!("new: ");
                            stdout().flush().expect("couldnt flush output");
                            stdin().read_line(&mut buff).expect("coudlnt get input");
                            t.sub_category = String::from(buff.trim_matches('\n'));
                        }
                        "3" => {
                            println!("current: {}", &t.priority);
                            print!("new(0-3): ");
                            stdout().flush().expect("couldnt flush output");
                            stdin().read_line(&mut buff).expect("coudlnt get input");
                            t.priority = match buff.trim_matches('\n').parse() {
                                Ok(p) => p,
                                Err(e) => {
                                    println!("ERROR: {:?}",e);
                                    continue;
                                }
                            }
                        }
                        "4" => {
                            let due = match &t.due {
                                Some(d) => {
                                    let d = d.to_localdate().unwrap();
                                    format!("{}/{}/{}",d.day(),d.month() as i16, d.year())
                                }
                                None => {
                                    String::from("None")
                                }
                            };

                            println!("current: {}",due);
                            print!("new(d/m/y): ");
                            stdout().flush().expect("couldnt flush output");
                            stdin().read_line(&mut buff).expect("coudlnt get input");
                            t.due = parse_to_date(&buff.trim_matches('\n'));
                        }
                        "5" => {
                            println!("current: {}", &t.done);
                            print!("new(t/f): ");
                            stdout().flush().expect("couldnt flush output");
                            stdin().read_line(&mut buff).expect("coudlnt get input");
                            t.done = buff.to_lowercase().starts_with('t');
                        }
                        _ => {
                            println!("invalid option");
                        },

                    }

                }                

            }
        }
        _ => {
            println!("{}", tasks.full_print_for_conky().trim());
        }
    }

    if should_save {
        let _ = tasks.save(&path);
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