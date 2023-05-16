mod app;
mod todo;
mod ui;


fn main() {
    let mut app = app::App::new();
    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // let filename = "todo.txt";
    // let mut todos = crate::todo::Todos::new(filename);
    // //
    // todos.load_from_file(filename);
    // todos.list_all_tasks();

    // // Adding todos
    // todos.add_todo("Call mom");
    // todos.add_todo("Do homework");
    // todos.add_todo("Sleep");
    //
    // // Marking a todo as done
    // todos.mark_as_done(2);
    //
    // // Listing todos
    // todos.list_all_tasks();
}
