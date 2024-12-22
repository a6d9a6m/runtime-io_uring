mod scheduler;
mod task;
mod io;
mod Proactor;
mod file;
use scheduler::Scheduler;
use task::Task;
use file;
fn main() {
    // 创建调度器
    let scheduler = Scheduler::new();
    let proactor= Proactor::new(8);
    let file = AsyncFile::open(proactor, &Path::new("test.txt")).unwrap();
    file.read().unwrap();
    // 添加任务
    scheduler.add_task(Task::new(1, "Task 1".to_string()));
    scheduler.add_task(Task::new(2, "Task 2".to_string()));
    scheduler.run();

}
