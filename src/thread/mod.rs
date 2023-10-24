use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use tokio::task;

async fn producer_consumer() -> Result<(), Box<dyn std::error::Error>> {
    
    let (mut tx, mut rx) = mpsc::channel::<i32>(10);
    let count = Arc::new(Mutex::new(0));

    let count_p = Arc::clone(&count);
    let producer = task::spawn(async move {
        loop {
            println!("1 time: {:?}", tokio::time::Instant::now());
            let mut num = count_p.lock().await;
            println!("2 time: {:?}", tokio::time::Instant::now());
            *num += 1;
            println!("3 time: {:?}", tokio::time::Instant::now());
            tx.send(*num).await.unwrap();
            println!("4 time: {:?}", tokio::time::Instant::now());
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("5 time: {:?}", tokio::time::Instant::now());
            println!("");
        }
    });

    let count_c = Arc::clone(&count);
    let consumer = task::spawn(async move {

        loop {
            if let Some(msg) = rx.recv().await {
                let mut num = count_c.lock().await;
                *num -= 1;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }

    });

    producer.await.unwrap();
    consumer.await.unwrap();

    Ok(())
}
