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


    match point {

        Point {x ,y:0} => println!("y =0"),
        Point {x:0 ,y} => println!("x =0"),
        Point{x,y} => println!("no 0")
    
    }


    let message = Message::ChangeColor(Color::Rgb(1, 3, 4));

    match message {
        Message::Quit => {
            println!("Quit");
        },
        Message::Move { x, y }=> {
            println!("Move {}, {}",x,y);
        },
        Message::Write ( msg )=> {
            println!("Write {}",msg);
        },
        Message::ChangeColor (Color::Rgb(r,g , b) )=> {
            println!("ChangeColor Rgb {}, {}, {}",r,g,b);
        },
        Message::ChangeColor (Color::Hsv(r,g , b) )=> {
            println!("ChangeColor Hsv  {}, {}, {}",r,g,b);
        },

        Message::ChangeColor (Color::Hsv(r,..) )=> {
            println!("ChangeColor Hsv  {}",r);
        }
    }

    let  s= Some(String::from("value"));
    
    if let Some(_) = s {
        println!("found a string")
    }

    let sum =Some(5);

    match sum {
        Some(x) if x<5 => println!("< 5"),
        Some(x) => println!("< 5"),
        _ => println!("")
    }

    println!("{:?}",s);
}
fn addTwo(_:i32,y:i32)-> i32{
    y+1
}
struct Point{
    x:i32,
    y:i32,
}

enum Color{
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32)
}
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(Color)
}

