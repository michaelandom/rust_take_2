fn main() {
    let x = Some(10);
    let y = 5;

    match x {
        Some(5) | Some(1)  => println!("1"),
        Some(2) => println!("3"),
        Some(y) => println!("y = {}",y),
        _ => println!("_")
    }

    println!("y = {}",y);


    let x2= 5;

    match x2 {
        1..=5 => println!("1-5"),
        7..=8 => println!("7-8"),   
        _ => println!("_")
    }
    
    let point = Point { x: 0, y:7};

    let Point{x:a, y:b} = point;

    assert_eq!(0,a);
    assert_eq!(7,b);


}

struct Point{
    x:i32,
    y:i32,
}

