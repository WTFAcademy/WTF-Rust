use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hello");
        tx.send(msg).unwrap();
        // 注意这里发送后，msg不能再访问，所有权已经转移到接收端
    });

    let received = rx.recv().unwrap(); // `recv`会阻塞主线程直到从通道接收到消息
    println!("Received: {}", received);
}
