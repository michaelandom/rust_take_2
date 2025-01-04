fn main() {
    let x = 8;
    let y: u32 = 8;

    let floating = 10.52;

    let true_or_false = true;

    let letter = 'a';

    let tup = (x, y, floating, true_or_false, letter);
    let mut tup2 = (x, y, floating, true_or_false, letter);
    tup2 = tup;

    tup2.0 = 19;
    println!("{}", tup2.0);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[0] = 2;

    let mut arr2: [i32; 5];

    println!("{}", arr[0]);
}
