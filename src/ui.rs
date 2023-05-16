use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    terminal::{self, Clear, ClearType},
    Result,
};
use std::io::{self, Write};

use crate::todo::Todos;


const DESCRIPTION_WIDTH: usize = 30;
const STATUS_WIDTH: usize = 10;
const INDEX_WIDTH: usize = 3;

const MODE_ROW: u16 = 1;
const HEADER_ROW: u16 = 3;

pub struct TodoUI {
    active_screen: TodoTab,
    cursor_row: usize,
}

pub enum TodoTab {
    TodoList,
    DoneList,
    UndoneList,
}

pub enum UiEvent {
    Quit,
    AddTodo,
    ChangeTab(TodoTab),
    RefreshUI,
}

pub trait Display {
    fn new() -> Self;
    fn initialize(&self) -> Result<()> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All), cursor::Hide)?;
        Ok(())
    }
}

impl Display for TodoUI {
    fn new() -> Self {
        Self {
            active_screen: TodoTab::TodoList,
            cursor_row: 0,
        }
    }
}

impl TodoUI {
    pub fn render(&self, todos: &Todos) -> Result<()> {
        let mut stdout = io::stdout();
        // Render the header
        let header = format!("{:^width$}", " All  Done  Undone ", width=DESCRIPTION_WIDTH+STATUS_WIDTH+INDEX_WIDTH+3);
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, MODE_ROW as u16),
            SetForegroundColor(Color::White),
            Print(&header),
            ResetColor,
        )?;
        let start_of_all = header.find("All").unwrap()-1;
        let start_of_done = header.find("Done").unwrap()-1;
        let start_of_undone = header.find("Undone").unwrap()-1;

        // Render the todos
        let format_header = format!("{:>INDEX_WIDTH$} | {:>DESCRIPTION_WIDTH$} | {:^STATUS_WIDTH$}","".repeat(INDEX_WIDTH),  "description", "status");
        let format_seperator = format!("{:>INDEX_WIDTH$}-|-{:>DESCRIPTION_WIDTH$}-|-{:^STATUS_WIDTH$}","-".repeat(INDEX_WIDTH),  "-".repeat(DESCRIPTION_WIDTH), "-".repeat(STATUS_WIDTH));
        // let execute_header = |""
        match self.active_screen {
            TodoTab::TodoList => {
                execute!(
                    stdout,
                    cursor::MoveTo(start_of_all as u16, MODE_ROW),
                    SetForegroundColor(Color::Blue),
                    Print("[All]"),
                    cursor::MoveTo(0, HEADER_ROW),
                    ResetColor,
                    Print(format_header),
                    cursor::MoveToNextLine(1),
                    Print(format_seperator),
                    cursor::MoveToNextLine(1),
                )?;
                self.render_todos(&mut stdout, todos, None)?;
            },
            TodoTab::DoneList => {
                execute!(
                    stdout,
                    cursor::MoveTo(start_of_done as u16, MODE_ROW),
                    SetForegroundColor(Color::Cyan),
                    Print("[Done]"),
                    cursor::MoveTo(0, HEADER_ROW),
                    ResetColor,
                    Print(format_header),
                    cursor::MoveToNextLine(1),
                    Print(format_seperator),
                    cursor::MoveToNextLine(1),
                )?;
                self.render_todos(&mut stdout, todos, Some(true))?;
            },
            TodoTab::UndoneList => {
                execute!(
                    stdout,
                    cursor::MoveTo(start_of_undone as u16, MODE_ROW),
                    SetForegroundColor(Color::Red),
                    Print("[Undone]"),
                    cursor::MoveTo(0, HEADER_ROW),
                    ResetColor,
                    Print(format_header),
                    cursor::MoveToNextLine(1),
                    Print(format_seperator),
                    cursor::MoveToNextLine(1),
                )?;
                self.render_todos(&mut stdout, todos, Some(false))?;
            },
        }

        // Render the footer
        execute!(stdout, ResetColor).unwrap();
        // self.render_prompt(&mut stdout)?;

        stdout.flush()?;
        Ok(())
    }

    // fn render_one_todo(&self, stdout: &mut Stdout, todo: &Todo) -> Result<()> {
    //     // terminal::EnterAlternateTodoTab;
    //     execute!(
    //         stdout,
    //         SetForegroundColor(Color::White),
    //         ResetColor,
    //         Print(format!("[{}] {}", if todo.is_done() {"x"} else {" "}, todo.description)),
    //     )?;
    //     Ok(())
    // }

    pub fn read_event(&mut self, todos: &mut Todos) -> Result<Option<UiEvent>> {
        if let Ok(event) = event::read() {
                // Handle key events
                match event {
                    Event::Key(event) => match event.code {
                        // Quit
                        KeyCode::Char('q') => return Ok(Some(UiEvent::Quit)),
                        // Add todo
                        KeyCode::Char('a') => {
                            execute!(io::stdout(), cursor::Show, cursor::EnableBlinking).unwrap();
                            terminal::disable_raw_mode().unwrap();
                            execute!(io::stdout(), cursor::MoveToNextLine(1), Print(">> ")).unwrap();
                            return Ok(Some(UiEvent::AddTodo))
                        }
                        // Remove todo
                        KeyCode::Char('d') => {
                           todos.remove_todo(self.cursor_row, match self.active_screen {
                               TodoTab::TodoList => None,
                               TodoTab::DoneList => Some(true),
                               TodoTab::UndoneList => Some(false),
                           });
                           return Ok(Some(UiEvent::RefreshUI));
                        }
                        // Navigate tabs
                        KeyCode::Char('l') | KeyCode::Right => {
                            self.cursor_row = 0;
                            let next_screen = match self.active_screen {
                                TodoTab::TodoList => TodoTab::DoneList,
                                TodoTab::DoneList => TodoTab::UndoneList,
                                TodoTab::UndoneList => TodoTab::TodoList,
                            };
                            return Ok(Some(UiEvent::ChangeTab(next_screen)));
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            self.cursor_row = 0;
                            let next_screen = match self.active_screen {
                                TodoTab::TodoList => TodoTab::UndoneList,
                                TodoTab::DoneList => TodoTab::TodoList,
                                TodoTab::UndoneList => TodoTab::DoneList,
                            };
                            return Ok(Some(UiEvent::ChangeTab(next_screen)));
                        }
                        // Select todo (up/down)
                        KeyCode::Char('j') | KeyCode::Down => {
                            match self.active_screen {
                                TodoTab::TodoList => {
                                    if self.cursor_row < todos.get_number_of_tasks(None) - 1 {
                                        self.cursor_row += 1;
                                    }
                                }
                                TodoTab::DoneList => {
                                    if self.cursor_row < todos.get_number_of_tasks(Some(true)) - 1 {
                                        self.cursor_row += 1;
                                    }
                                }
                                TodoTab::UndoneList => {
                                    if self.cursor_row < todos.get_number_of_tasks(Some(false)) - 1 {
                                        self.cursor_row += 1;
                                    }
                                }
                            }
                            return Ok(Some(UiEvent::RefreshUI));
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if self.cursor_row > 0 {
                                self.cursor_row -= 1;
                            }
                            return Ok(Some(UiEvent::RefreshUI));
                        }
                        // Mark as done/undone
                        KeyCode::Char('x') => {
                            match todos.get_task(self.cursor_row, match self.active_screen {
                                    TodoTab::TodoList => None,
                                    TodoTab::DoneList => Some(true),
                                    TodoTab::UndoneList => Some(false),
                                }
                            ){

                                Some(todo) => {
                                    match todo.is_done() {
                                        true => todo.mark_as_undone(),
                                        false => todo.mark_as_done(),
                                    };
                                },
                                None => {},
                            }
                            return Ok(Some(UiEvent::RefreshUI));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(None)
    }


    pub fn change_screen(&mut self, screen: TodoTab) {
        self.active_screen = screen;
    }

    pub fn refresh_screen(&mut self, todos: &Todos) {
        let cursor_row = match self.active_screen {
            TodoTab::TodoList => todos.get_number_of_tasks(None).saturating_sub(1),
            TodoTab::DoneList => todos.get_number_of_tasks(Some(true)).saturating_sub(1),
            TodoTab::UndoneList => todos.get_number_of_tasks(Some(false)).saturating_sub(1),
        };
        self.cursor_row = self.cursor_row.min(cursor_row);
    }

    pub fn add_todo(&self) {
        execute!(io::stdout(), cursor::Hide, cursor::DisableBlinking).unwrap();
        terminal::enable_raw_mode().unwrap();
    }


    fn render_todos(&self, stdout: &mut io::Stdout, todos: &Todos, done: Option<bool>) -> Result<()> {
        let format_string = |index: usize, description: &str, status: &str| -> String {
            let truncated_desc = if description.len() > (DESCRIPTION_WIDTH - 3) {
                format!("{}...", &description[..DESCRIPTION_WIDTH - 3])
            } else {
                description.to_string()
            };
            format!("{:>INDEX_WIDTH$} | {:>width$} | {:^STATUS_WIDTH$}", index+1, truncated_desc, status, width=DESCRIPTION_WIDTH)
        };

        let todos = todos.get_tasks(done);
        for (index, todo) in todos.iter().enumerate() {
            execute!(
                stdout,
                SetForegroundColor(Color::White),
                SetBackgroundColor(if self.cursor_row == index { Color::Blue } else { Color::Black }),
                Print(format_string(index, &todo.description, if todo.done { "[x]" } else { "[ ]" })),
                // Print(format!("{:>3}. {:>30} [{:^10}]", index + 1, todo.description, if todo.done { "x" } else { " " })),
                cursor::MoveToNextLine(1),
            )?;
        }
        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        let mut stdout = io::stdout();
        execute!(
            stdout,
            ResetColor,
            cursor::Show,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0),
        )?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn read_line(&self) -> io::Result<String> {
        let mut line = String::new();
        while let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char(c) => {
                    line.push(c);
                }
                _ => {}
            }
        }

        Ok(line)
    }
}
