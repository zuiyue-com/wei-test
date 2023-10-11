use std::fs::File;
use std::io::Write;
use rand::Rng;

fn main() -> std::io::Result<()> {
    let mut file = File::create("r")?;
    let mut rng = rand::thread_rng();

    for _ in 0..(10 * 1024) {
        let mut buffer = vec![0u8; 1024];
        rng.fill(&mut buffer[..]);
        file.write_all(&buffer)?;
    }

    Ok(())
}