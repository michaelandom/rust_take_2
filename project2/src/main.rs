use std::fs::File;
use std::io::prelude::*;
fn main() -> std::io::Result<()> {
    let file_name = "foo.txt";
    let mut file = File::create(file_name)?;
    file.write_all(b"hgghghhghg")?;
    let mut file_open = File::open(file_name)?;
    let mut contents = String::new();
    file_open.read_to_string(&mut contents)?;
    println!("read {}", contents);
    Ok(())
}
