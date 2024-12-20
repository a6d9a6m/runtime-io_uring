use crate::task::Task;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Scheduler {
    tasks: Arc<Mutex<Vec<Task>>>,  // 存储待调度的任务
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(vec![])),
        }
    }

    // 向调度器添加任务
    pub fn add_task(&self, task: Task) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(task);
    }

    // 运行所有任务
    pub fn run(&self) {
        loop {
            let tasks = self.tasks.lock().unwrap();
            for task in tasks.iter() {
                println!("Running task: {} - {}", task.id, task.name);
                // 模拟协程的执行
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
