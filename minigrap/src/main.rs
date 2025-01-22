use std::env;
fn main() {
    let arg: Vec<String>= env::args().collect();
    println!("{:?}",arg);

    let query = &arg[1];
    let filename = &arg[2];


    println!("{:?}",query);
    println!("{:?}",filename);


}
