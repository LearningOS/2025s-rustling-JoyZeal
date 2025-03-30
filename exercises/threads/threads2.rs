// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 使用 Arc 和 Mutex 包装 JobStatus 结构体，以实现多线程间的安全共享
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆 Arc 指针，使每个线程都能引用共享数据
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            // 模拟线程工作，休眠 250 毫秒
            thread::sleep(Duration::from_millis(250));
            // 加锁，确保同一时间只有一个线程能修改共享数据
            let mut status_lock = status_shared.lock().unwrap();
            // 更新共享数据
            status_lock.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成工作
    for handle in handles {
        handle.join().unwrap();
    }

    // 再次加锁以读取共享数据
    let final_status = status.lock().unwrap();
    // 打印最终完成的任务数量
    println!("jobs completed {}", final_status.jobs_completed);
}    
