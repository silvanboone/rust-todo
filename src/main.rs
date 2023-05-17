use app::App;
use manager::FileManager;
use task::Task;

mod app;
mod manager;
mod task;

fn main() {
    let file_manager = FileManager::new("./tasks.json".to_string());

    let mut app: App = App::new(file_manager.get_tasks());

    app.delete_task("a".to_string(), None, None);

    file_manager.write_tasks(app.task_list);
}
