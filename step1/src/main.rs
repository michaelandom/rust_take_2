// Variables, Constants and Shadowing
fn main() {

    let x = 4;
    const  Y: i32 = 1;
    println!("x is {}", x);
    let x = 5;
    println!("x is {}", x);
    {
        let x = 2;
        println!("x is {}",x);
    }

    {
        let x = x - 2;
        println!("x is {}",x);
    }
    let x = x + 2;
    println!("x is {}", x);


    let x = "hello";
    println!("x is: {}", x)
}
