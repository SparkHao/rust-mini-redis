
async fn say_hello(){
    println!("world");
}

#[tokio::main]
async fn main() {
    let op = say_hello();
    println!("hello");
    op.await;
}