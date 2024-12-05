mod task;
use task::domain::task_entity;
fn main() {
    let task = task_entity::Task::create_task();
    println!("Task: {}", task.title);
}
