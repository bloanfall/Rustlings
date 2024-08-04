use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // 使用 Arc<Mutex<JobStatus>> 来允许线程安全地共享和修改状态。
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        // 每个线程都需要克隆 Arc 来增加引用计数。
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // 在修改数据之前，我们需要锁定 Mutex。
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // 等待所有工作线程完成。
    for handle in handles {
        handle.join().unwrap();
    }

    // 获取最终的 jobs_done 值，并在锁定 Mutex 后打印它。
    let job_status = status.lock().unwrap();
    println!("Jobs done: {}", job_status.jobs_done);
}
