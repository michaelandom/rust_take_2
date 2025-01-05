

fn sum(num1: i32, num2: i32) -> i32{
    println!("sum is {}", num1 + num2);
    num1 + num2
}

fn largest(num1: i32, num2: i32,num3: i32) -> i32{
    let mut largest = num1;

    if largest < num2 {
        largest = num2;
    }
    if largest < num3 {
        largest = num3;
    }
    largest
}
fn main() {
    println!("main called!");
    test();
    sum(2000000000,4);
    largest(5,4,7);

    let number = {
        let x = 4;
        let y = 5;
        if x > y {
            x
        } else {
            y
        }
    };

    println!("number called number  {} ", number);

}

fn test(){
    println!("test fn code called")
}