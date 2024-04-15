// 引入所需的库
use serde::{Serialize, Deserialize};
use std::error::Error;

// 定义一个序列化和反序列化的结构体
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_member: bool,
}

// 异步主函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建Person实例
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_member: true,
    };

    // 序列化Person实例到JSON字符串
    let serialized = serde_json::to_string(&person)?;
    println!("serialized = {}", serialized);

    // 将JSON字符串反序列化回Person结构体实例
    let deserialized: Person = serde_json::from_str(&serialized)?;
    println!("deserialized = {:?}", deserialized);

    // 发起网络请求并获取响应
    let resp = reqwest::get("https://mock.apifox.com/m1/4333142-0-default/user")
        .await?
        .text()
        .await?;

    // 打印原始响应数据
    println!("Response as text: {}", resp);

    // 将响应的JSON字符串反序列化为Person结构体
    let person: Person = serde_json::from_str(&resp)?;

    // 打印反序列化后的Person结构体
    println!("Deserialized person: {:?}", person);

    Ok(())
}