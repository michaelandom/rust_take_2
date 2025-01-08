fn main() {
    let x = 4;
    let y = x; // copy

    let x1 = String::from("value");
    let y1 = x1; // move
    let y3 = y1.clone();
    println!("y1 this is {}", y1);

    let a = String::from("value");
    take_owner_string(a);
    // println!("a is {}",a); b/c there was a move 
    let num = 5;
    take_owner_number(num);
    println!("num is {}", num);
    let new_string= give_ownership();
    println!("new_string is {}", new_string);

    // get_length(new_string);
    // println!("after get_length is {}", new_string);
    without_ownership_get_length(&new_string);
    println!("after get_length is {}", new_string);




    let mut s = String::from("Ss");

    let s1 = & s;
    let s2 = & s;
    // let s2 = &mut s; // cannot borrow `s` as mutable more than once at a time
    //let s3 = &s;

    println!("print s1 {s1} s2 {s2}");

    let s3 = &mut s;

    println!("print s3 {s3}");







}
fn take_owner_string(some_string: String) {
    println!("some_string is {}", some_string);
}
fn take_owner_number(some_number: i32) {
    println!("some_number is {}", some_number);
}
fn give_ownership() -> String {
    let a= String::from("aa");
    a
}

fn get_length(some_string: String) -> (String,usize) {
    let length: usize = some_string.len();
    (some_string,length)
}

fn without_ownership_get_length(some_string: &String) -> usize {
    let length: usize = some_string.len();
    length
}