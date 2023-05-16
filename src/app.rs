use crate::todo::Todos;
use crate::ui::{TodoUI, UiEvent};
use std::error::Error;
use crate::ui::Display;

pub struct App {
    todos: Todos,
    ui: TodoUI,
}

impl App {
    pub fn new() -> Self {
        let file_name = "todo.txt";
        let mut todos = Todos::new(file_name);
        todos.load_from_file(file_name);

        let ui = TodoUI::new();

        Self { todos, ui }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.ui.initialize()?;
        loop {
            self.ui.render(&self.todos)?;
            if let Some(event) = self.ui.read_event(&mut self.todos)? {
                self.handle_event(event)?;
            }
        }
    }

    fn handle_event(&mut self, event: UiEvent) -> Result<(), Box<dyn Error>> {
        match event {
            UiEvent::Quit => {
                self.ui.cleanup()?;
                std::process::exit(0);
            },
            UiEvent::AddTodo => {
                let input = self.ui.read_line()?;
                self.todos.add_todo(&input);
                self.ui.add_todo();

            },
            UiEvent::ChangeTab(screen) => {
                self.ui.change_screen(screen);
            },
            UiEvent::RefreshUI => {
                self.ui.refresh_screen(&self.todos);
            },
        }
        Ok(())
    }
}
