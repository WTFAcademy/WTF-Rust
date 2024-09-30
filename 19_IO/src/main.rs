use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut file = File::open("example.txt").await.expect("Failed to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.expect("Failed to read data");

    println!("File contents: {:?}", String::from_utf8(contents).expect("Data not UTF-8"));
}
