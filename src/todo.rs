use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ptr::eq;


pub struct Todo {
    pub description: String,
    pub done: bool,
}

impl Todo {
    fn new(description: &str) -> Self { 
        Self { 
            description: String::from(description), 
            done: false 
        } 
    }

    pub fn mark_as_done(&mut self) {
        self.done = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn mark_as_undone(&mut self) {
        self.done = false;
    }
}
pub struct Todos {
    pub todos: Vec<Todo>,
    pub filesave: String,
}

impl Todos {
    pub fn new(filename: &str) -> Self {
        Todos {
            todos: Vec::new(),
            filesave: String::from(filename),
        }
    }

    pub fn add_todo(&mut self, description: &str) {
        let new_todo = Todo::new(description);
        self.todos.push(new_todo);
        self.save_to_file();
    }

    pub fn remove_todo(&mut self, index: usize, done: Option<bool>) {
        let task = match done {
            Some(true) => self.todos.iter().filter(|todo| todo.is_done()).nth(index),
            Some(false) => self.todos.iter().filter(|todo| !todo.is_done()).nth(index),
            None => self.todos.get(index),
        };
        match task {
            Some(task) => {
                let index = self.todos.iter().position(|x| eq(x, task)).unwrap();
                self.todos.remove(index);
                self.save_to_file();
            },
            None => (),
        }
    }

    pub fn save_to_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.filesave)
            .unwrap();

        for todo in &self.todos {
            let status = if todo.is_done() { "x" } else { " " };
            writeln!(file, "{},{}", status, todo.description).unwrap();
        }
    }

    pub fn get_task(&mut self, index: usize, done: Option<bool>) -> Option<&mut Todo> {
        match done {
            Some(true) => self.todos.iter_mut().filter(|todo| todo.is_done()).nth(index),
            Some(false) => self.todos.iter_mut().filter(|todo| !todo.is_done()).nth(index),
            None => self.todos.get_mut(index),
        }
    }

    pub fn get_tasks(&self, done: Option<bool>) -> Vec<&Todo> {
        match done {
            Some(true) => self.todos.iter().filter(|todo| todo.is_done()).collect::<Vec<_>>(),
            Some(false) => self.todos.iter().filter(|todo| !todo.is_done()).collect::<Vec<_>>(),
            None => self.todos.iter().collect::<Vec<_>>(),
        }
    }
    pub fn get_number_of_tasks(&self, done: Option<bool>) -> usize {
        match done {
            Some(true) => self.todos.iter().filter(|todo| todo.is_done()).count(),
            Some(false) => self.todos.iter().filter(|todo| !todo.is_done()).count(),
            None => self.todos.len(),
        }
    }

    pub fn load_from_file(&mut self, filename: &str) {
        let file = File::open(filename);
        match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                let mut todos: Vec<Todo> = vec![];
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let splitted = line.split(",").collect::<Vec<&str>>();
                        todos.push(
                            Todo {
                                description: splitted[1].to_string(),
                                done: if splitted[0] == "x" { true } else { false },
                            }
                        );
                    }
                }
                self.todos = todos;
                self.filesave = filename.to_string();
            },
            Err(e) => eprintln!("Error: {}", e)
        }
    }
    // pub fn list_all_tasks(&self) {
    //     self.print_task_list(None);
    // }
    //
    // pub fn list_done_tasks(&self) {
    //     self.print_task_list(Some(true))
    // }
    //
    // pub fn list_undone_tasks(&self) {
    //     self.print_task_list(Some(false))
    // }
    //
    // pub fn print_task_list(&self, filter_done: Option<bool>) {
    //     if self.todos.is_empty() {
    //         println!("Task list is empty!")
    //     } else {
    //         const DESCRIPTION_SPACE: usize = 30;
    //         const STATUS_SPACE: usize = 7;
    //         const INDEX_SPACE: usize = 3;
    //
    //         let format_header_print = |description: &str, status: &str| -> String {
    //             format!("{:>DESCRIPTION_SPACE$}|{:^STATUS_SPACE$}", description, status)
    //         };
    //         let format_body_print = |index: usize, description: &str, status: &str| -> String {
    //             format!("{:>INDEX_SPACE$}. {:<width$}|{:^STATUS_SPACE$}", index+1, description, status, width=DESCRIPTION_SPACE-INDEX_SPACE-2)
    //         };
    //
    //         // Print time 
    //         println!("{}", format_header_print("DESCRIPTION ", " STATUS"));
    //         println!("{}", format_header_print("-".repeat(DESCRIPTION_SPACE).as_str(), "-".repeat(STATUS_SPACE).as_str()));
    //         for (idx, todo) in self.todos.iter().enumerate() {
    //             match filter_done {
    //                 Some(true) => {
    //                     if todo.is_done() {
    //                         println!("{}", format_body_print(idx, todo.description.as_str(), if todo.done {"[x]"} else {"[ ]"}));
    //                     }
    //                 },
    //                 Some(false) => {
    //                     if !todo.is_done() {
    //                         println!("{}", format_body_print(idx, todo.description.as_str(), if todo.done {"[x]"} else {"[ ]"}));
    //                     }
    //                 },
    //                 None => {
    //                     println!("{}", format_body_print(idx, todo.description.as_str(), if todo.done {"[x]"} else {"[ ]"}));
    //                 }
    //             }
    //         }
    //
    //     }
    // }

}

