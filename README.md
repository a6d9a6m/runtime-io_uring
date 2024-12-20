# 说明文档


项目结构
```
coroutine_runtime/
├── src/
│   ├── main.rs          # 启动协程和主调度器
│   ├── scheduler.rs     # 协程调度器模块
│   ├── task.rs          # 协程任务定义
│   └── io.rs            # io_uring 操作模块
├── Cargo.toml           # 项目依赖
```
