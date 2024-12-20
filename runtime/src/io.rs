use io_uring::IoUring;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::io::{self, Write};

pub fn perform_io_uring_operation() -> io::Result<()> {
    let ring = IoUring::new(256)?;  // 创建 io_uring 实例，256 个条目

    // 打开文件
    let mut file = File::create("output.txt")?;

    // 获取提交队列条目（从提交队列获取一个提交条目）
    let mut sqe = ring.; // 获取提交队列中的下一个条目

    // 配置提交队列条目，准备执行写操作
    sqe.prep_write(file.as_raw_fd(), b"Hello, io_uring!", 0);

    // 提交请求并等待完成
    ring.submit_and_wait(1)?;

    println!("Async I/O operation completed!");

    Ok(())
}
