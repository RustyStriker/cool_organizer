use serde_derive::{Serialize,Deserialize};
use datetime::{LocalDate, convenience::Today, DatePiece};
use std::{error::Error, fs};

#[derive(Serialize,Deserialize)]
pub struct TasksManager {
    pub colors : FormatParams,
    pub tasks : Vec<Task>,
}
impl TasksManager {
    pub fn default() -> TasksManager {
        TasksManager {
            colors : FormatParams {
                default: String::new(),
                category: String::new(),
                sub_category: Some(String::new()),
                prio_1: String::new(),
                prio_2: String::new(),
                prio_3: String::new(),
                show_days_forward : -1,
            },
            tasks: Vec::new(),
        }
    }

    pub fn save(&self, path : &str) -> Result<(), impl Error> {
        let toml = toml::to_string(self).unwrap();

        let _ = fs::File::create(path);
        fs::write(path, toml)
    }

    pub fn full_print_for_conky(&self) -> String {
        let tasks = &self.tasks;
        let colors = &self.colors;

        let mut res = String::new();
    
        let mut cats = Vec::new();
    
        // Extract the categories first
        for task in tasks {
            if !cats.contains(&task.category) {
                cats.push(task.category.clone());
            }
        }
        cats.sort();
    
        for cat in cats {
            // Iterate over our categories, find the wanted tasks and sort them in an array based on prio
            let mut group : Vec<&Task> = Vec::new();
    
            let mut gs = String::new();

            for task in tasks {
                if task.category == cat {
                    group.push(&task);
                }
            }
            group.sort_by_key(|a| a.priority);
            group.reverse();
    
            // Begin to print the stuff
            gs.push_str(&format!("${{{}}}{}:\n", colors.category, cat));
    
            let mut inserted = false;

            for t in group {
                let t = t.formatted_conky(&colors, true);
                if !t.is_empty(){
                    inserted = true;
                    gs.push_str(&format!(" {}\n", t));
                }
            }
            // push another line break to add some gap
            if inserted {
                res.push_str(&gs);
                res.push('\n');
            }
        }
    
        res
    }
}

#[derive(Clone,Serialize,Deserialize, Copy)]
pub struct Date {
    pub year : i64,
    pub day : i64,
}
impl Date{
    pub fn ymd(year : i64, day : i64) -> Date {
        Date{
            year,day
        }
    }
    pub fn from(date : LocalDate) -> Date {
        Date {
            year : date.year(),
            day : date.yearday() as i64,
        }
    }
    pub fn to_localdate(&self) -> Option<LocalDate> {
        match LocalDate::yd(self.year, self.day) {
            Ok(d) => Some(d),
            Err(_) => None
        }
    }
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Task {
    pub category : String,
    pub sub_category : String,
    pub priority : u8,
    pub name : String,
    pub done : bool,
    pub due : Option<Date>,
}
impl Task {
    // Creators
    pub fn new(name : &str) -> Task {
        Task {
            category : String::new(),
            sub_category : String::new(),
            priority : 0,
            name : String::from(name),
            due : None,
            done : false
        }
    }
    pub fn due(self, due : Option<Date>) -> Task {
        Task {
            due,
            ..self
        }
    }
    pub fn category(self, category : &str) -> Task {
        Task {
             category : String::from(category),
             ..self
        }
    }
    pub fn sub_category(self, sub_category : &str) -> Task {
        Task {
            sub_category : String::from(sub_category),
            ..self
        }
    }
    pub fn priority(self, priority : u8) -> Task {
        Task {
            priority,
            ..self
        }
    }
    pub fn done(self, done : bool) -> Task {
        Task {
            done,
            ..self
        }
    }

    pub fn days_remianing(&self) -> Option<i16> {
        let today = LocalDate::today();

        let due = match self.due {
            Some(due) => due.to_localdate().unwrap(),
            None => {return None;}
        };

        let due = due.yearday() + if due.year() == today.year() + 1 { 365 } else { 0 };

        Some(due - today.yearday())
    }

    pub fn formatted(&self, sub : bool) -> String {
        // Category:
        //      - [x] Task (sub_cat) - due in X days for d/m
        
        let mut s = String::from("- ");

        if self.done {
            s.push_str("[x] ");
        }
        s.push_str(&self.name);

        if !self.sub_category.is_empty() && sub {
            s.push_str(&format!(" ({})",self.sub_category.trim()));
        }

        // Until here : '- [x] Task ( sub_cat )'

        if let Some(due) = self.due {
            let due = due.to_localdate().unwrap();
            let month = due.month();
            

            s.push_str(&format!(" - due in {} days for {}/{}", self.days_remianing().unwrap_or(0), due.day(), month as i32));
        }

        s
    }

    pub fn formatted_conky(&self, colors : &FormatParams, sub : bool) -> String {
        // Category:
        //      - [x] Task (sub_cat) - due in X days for d/m
        
        let c = match self.priority {
            1 => &colors.prio_1,
            2 => &colors.prio_2,
            3 => &colors.prio_3,
            _ => &colors.default
        };

        let days = self.days_remianing().unwrap_or(-1);
        if days > colors.show_days_forward && colors.show_days_forward > 0 {
            return String::new();
        }

        let mut s = String::from(&format!("${{{}}}- ", c));
        //                                       ^^^^^^ if you do {{}} it treats is a written explicitly {}(so it doesnt replace it)
        

        if self.done {
            s.push_str("[x] ");
        }
        s.push_str(&self.name);

        if !self.sub_category.is_empty() && sub {
            let co = if let Some(col) = &colors.sub_category { col } else { &colors.default };
            s.push_str(&format!(" (${{{}}}{}${{{}}})",co, self.sub_category.trim(),c));
        }

        // Until here : '- [x] Task ( sub_cat )'

        if let Some(due) = self.due {
            let due = due.to_localdate().unwrap();

            let month = due.month();
            
            s.push_str(&format!("${{alignr}} - due in {} days for {}/{}", days, due.day(), month as i32));
            
        }

        s
    }
}
#[derive(Clone,Serialize,Deserialize)]

pub struct FormatParams {
    pub default : String,
    pub category : String,
    pub prio_1 : String,
    pub prio_2 : String,
    pub prio_3 : String,
    pub show_days_forward : i16,
    pub sub_category : Option<String>,
}