fn main() {
    let x = Some(5);
    let y = 5;

    match x {
        Some(1)  => println!("1"),
        Some(2) => println!("3"),
        Some(y) => println!("y = {}",y),
        _ => println!("_")
    }

    println!("y = {}",y);
}
