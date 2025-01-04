use std::io;
fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}",input);

    let x = 5 as i64;
    let y = 10 as i32;



    let answer = x /( y as i64 );

    println!("{}", answer);

    input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input +2);



}
