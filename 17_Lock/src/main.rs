use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    test_mutex();
    test_rwlock();
    demo_deadlock();
    // demo_rwlock_upgrade(); // 由于标准库不支持锁升级，这部分代码作为说明用
}

fn test_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result from Mutex: {}", *counter.lock().unwrap());
}

fn test_rwlock() {
    let data = Arc::new(RwLock::new(5));
    let reader = Arc::clone(&data);
    let writer = Arc::clone(&data);

    let reader_thread = thread::spawn(move || {
        let read_data = reader.read().unwrap();
        println!("Reader got: {}", read_data);
    });

    let writer_thread = thread::spawn(move || {
        let mut write_data = writer.write().unwrap();
        *write_data += 1;
        println!("Writer updated data to: {}", *write_data);
    });

    reader_thread.join().unwrap();
    writer_thread.join().unwrap();

    println!("Final result from RwLock: {}", *data.read().unwrap());
}

fn demo_deadlock() {
    let resource_a = Arc::new(Mutex::new(42));
    let resource_b = Arc::new(Mutex::new("Rust"));

    let a_clone = Arc::clone(&resource_a);
    let b_clone = Arc::clone(&resource_b);

    let handle1 = thread::spawn(move || {
        let a = resource_a.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let b = resource_b.lock().unwrap();
        println!("Thread 1 acquired resources: {} and {:?}", a, b);
    });

    let handle2 = thread::spawn(move || {
        let b = b_clone.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        let a = a_clone.lock().unwrap();
        println!("Thread 2 acquired resources: {} and {:?}", a, b);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// 此函数仅作解释用，实际Rust标准库不支持锁升级
// fn demo_rwlock_upgrade() {
//     let lock = Arc::new(RwLock::new(1));
//     let lock_clone = Arc::clone(&lock);

//     let handle = thread::spawn(move || {
//         let read_handle = lock.read().unwrap();
//         println!("Read-lock acquired: {}", *read_handle);
//         // 假设在此处需要锁升级
//         let mut write_handle = lock.write().unwrap(); // 实际这里不能这样使用，会造成死锁
//         *write_handle += 1;
//     });

//     handle.join().unwrap();
//     println!("Value after supposed lock upgrade: {}", *lock.read().unwrap());
//}
