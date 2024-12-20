mod scheduler;
mod task;
mod io;

use scheduler::Scheduler;
use task::Task;
use io::perform_io_uring_operation;

fn main() {
    // 创建调度器
    let scheduler = Scheduler::new();

    // 添加任务
    scheduler.add_task(Task::new(1, "Task 1".to_string()));
    scheduler.add_task(Task::new(2, "Task 2".to_string()));

    // 运行协程调度器
    scheduler.run();

    // 执行 io_uring 异步操作
    match perform_io_uring_operation() {
        Ok(()) => println!("I/O operation completed."),
        Err(e) => eprintln!("I/O operation failed: {:?}", e),
    }
}
