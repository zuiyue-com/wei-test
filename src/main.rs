#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    println!("args: {:?}", args);

    let mut command = String::new();
    if args.len() > 1 {
        command = args[1].clone();
    }

    match command.as_str() {
        "t3" => { 
            t3().await?;
        }
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
                
                if key.starts_with("tree_1") {
                    continue;
                }
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

    let subscriber = db.watch_prefix("GPU_");

    let db_1 = db.clone();
    let thread1 = std::thread::spawn(move || {
        db.insert("GPU_1", b"456").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(3));
        db.insert("GPU_2", b"456").unwrap();
    });

    std::thread::spawn(move || {
        db_1.remove("GPU_2").unwrap();
    });

    std::thread::spawn(move || {
    
            for event in subscriber.take(2) {
                match event {
                    sled::Event::Insert{ key, value } => {
                        println!("insert {:?} {:?}", key, value);
                    },
                    sled::Event::Remove {key } => {
                        println!("remove {:?}", key);
                    }
                }
            }
            
    
    });

    thread1.join().unwrap();
    Ok(())
}

async fn t3() -> Result<(), Box<dyn std::error::Error>> {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..10 {
            //let val = String::from("hello");
            tx.send(i).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        
        }
        println!("finished tx");
    });

    let thread = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(val) => {
                    println!("Got: {}", val);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    println!("End: {}", val);
                }
                Err(_) => continue,
            }
        }
    });

    thread.join().unwrap();
    Ok(())
}