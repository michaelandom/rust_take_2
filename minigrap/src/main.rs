use std::env;
use std::fs;
fn main() {
    let arg: Vec<String>= env::args().collect();
    println!("{:?}",arg);

    let query = &arg[1];
    let filename = &arg[2];


    println!("search word {}",query);
    println!("file {}",filename);

    let contant = fs::read_to_string(filename).expect("error no file");
    
    println!("with text:\n{}", contant);

}
