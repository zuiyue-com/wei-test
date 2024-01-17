fn main() -> Result<(), Box<dyn std::error::Error>> {
    // wei_file::xz_decompress("C:/Users/Wei/Desktop/work/wei-release/wei/windows/1.1.19/a.tar.xz")?;

    wei_file::xz_compress("C:/Users/Wei/Desktop/work/wei-release/wei/windows/1.1.18")?;
    Ok(())
}