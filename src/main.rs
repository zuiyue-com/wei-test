use std::thread;
use std::time::Duration;

fn main() {
    println!("start");
    test();
    
    println!("end");
    thread::sleep(Duration::from_secs(10));
}

fn test() {
    // 创建并运行第一个线程
    thread::spawn(|| {
        println!("Starting another thread...");

        // 在第一个线程中创建并运行第二个线程
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(5));
            println!("Finished in the second thread");
        });

        println!("Finished in the first thread");
    });

    println!("exit test");
}