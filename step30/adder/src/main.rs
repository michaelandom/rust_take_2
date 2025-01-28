use add_one;
use add_two;
use rand;
fn main() {
    let num =2;
    println!("num {} and plus one {}",num,add_one::add_one(num));
    println!("num {} and plus one {}",num,add_two::add_two(num));
}
