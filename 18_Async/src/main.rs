use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct SimpleFuture;
impl Future for SimpleFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("completed".to_string())
    }
}


// 定义一个异步函数
async fn perform_task() -> String {
    "hello async".to_string()
}

#[tokio::main]
async fn main() {
    // 调用一个异步函数并等待它的完成
    let async_function_result = perform_task().await;
    println!("Async function result: {}", async_function_result);

    // 使用自定义的 Future 并等待它的完成
    let custom_future_result = SimpleFuture.await;
    println!("Custom Future result: {}", custom_future_result);
}

