fn main() {
    let a = 5;
    let b = &mut a;

    let mut c =10;
    let d = &c;
    println!("d {}",*d);
    *d = 20; //cannot assign to `*d`, which is behind a `&` reference  `d` is a `&` reference, so the data it refers to cannot be written
}
