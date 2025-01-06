use std::fs;
fn main() -> std::io::Result<()> {
    let file_name = "foo.txt";

    match write_data_to_file(&file_name, &"asdasd") {
        Ok(_) => println!("It worked"),
        Err(_) => println!("well... at least this demo held"),
    }

    match read_data_to_file(&file_name) {
        Ok(output) => println!("{}",output),
        Err(_) => println!("well... at least this demo held"),
    }
    Ok(())
}

fn write_data_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, &data)?;
    Ok(())
}

fn read_data_to_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let l = fs::read_to_string(&path)?;
    Ok(l)
}
