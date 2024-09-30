use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];

    // 创建多个线程
    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("Thread {} is running", i);
            // 模拟一些工作
            thread::sleep(Duration::from_secs(1));
            println!("Thread {} has finished", i);
        });
        handles.push(handle);
    }

    // 等待所有线程结束
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have finished.");
}
