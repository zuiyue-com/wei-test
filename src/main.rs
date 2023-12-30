use std::thread;
use std::time::Duration;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = "wsl";
    let command = "wei-docker-linux redis \"https://www.zuiyue.com/discuz/clientapi.php?modac=process&modac=process&time=1703922694&uuid=a6caaa78-be7f-4447-9883-a4d5bece541f&taskUuid=484\"";

    let output = match std::process::Command::new(cmd)
        .arg("-e") // 使用 `-e` 来执行指定的命令
        .arg("/bin/sh") // 指定使用shell
        .arg("-c") // 告诉shell执行后面的命令字符串
        .arg(command) // 传递整个命令字符串
        .output() {
            Ok(output) => output,
            Err(e) => {
                println!("Failed to execute process: {}", e);
                return Ok(());
            }
        };

    let data = format!("{}{}", 
        std::str::from_utf8(&output.stdout)?, 
        std::str::from_utf8(&output.stderr)?
    );

    println!("{}", data);

    Ok(())
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