#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    println!("args: {:?}", args);

    let mut command = String::new();
    if args.len() > 1 {
        command = args[1].clone();
    }

    match command.as_str() {
        "t1" => {
            t1().await?;
        },
        "t2" => t2().await?,
        _ => {
            let tree = sled::open("./welcome-to-sled")?;
            let mut iter = tree.iter();

            // 遍历所有的键值对
            while let Some(Ok((key, value))) = iter.next() {
                // `key` 和 `value` 都是 `IVec` 类型，你可能需要把它们转换成其他类型
                let key = String::from_utf8(key.to_vec()).unwrap();
                let value = String::from_utf8(value.to_vec()).unwrap();
                
                println!("{}: {}", key, value);
            }
        }
    }



    Ok(())
}

async fn t1() -> Result<(), Box<dyn std::error::Error>> {
    let tree = sled::open("./welcome-to-sled")?;

    let now = format!("{:?}", tokio::time::Instant::now());

    tree.insert(
        &*now,
        "key"
    )?;

    tokio::time::sleep(tokio::time::Duration::from_secs(65)).await;

    tree.flush()?;

    Ok(()) 

}

async fn t2() -> Result<(), Box<dyn std::error::Error>> {
    let db = sled::open("./welcome-to-sled")?;
    
    let mut subscriber = db.watch_prefix(vec![]);

    let tree_2 = db.clone();



    Ok(())
}