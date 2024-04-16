// 引入所需的库
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::error::Error as StdError;

// 定义一个序列化和反序列化的结构体
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    is_member: bool,
}

async fn fetch_and_parse_json(url: &str) -> Result<Value, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;
    Ok(json)
}

// 异步主函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
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


    // 假设对方 json 格式未知，则：
    let url = "https://mock.apifox.com/m1/4333142-0-default/orders"; // 替换为实际的接口URL
    match fetch_and_parse_json(url).await {
        Ok(json) => {
            // 这里可以动态地访问和解析JSON数据
            println!("{:?}", json);

            // 例如，检查某个键是否存在并获取对应的值
            if let Some(data) = json.get("some_key") {
                println!("Data under 'some_key': {:?}", data);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}