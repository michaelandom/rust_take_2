use colored::*;
#[derive(Debug)]
struct Rectangle {
    wight: u32,
    hight: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.wight * self.hight;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.hight > other.hight && self.wight > other.wight
    }

    fn can_hold_2(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}
impl Rectangle {
    
    fn square(size:u32)-> Rectangle{
        Rectangle{
            wight:size,
            hight:size
        }
    }
}

fn main() {
    let rect = Rectangle { wight: 5, hight: 5 };

    println!("{} {:#?}","rect".green(), rect);
    println!("rect area {:?}", rect.area());

    println!(
        "rect can_hold {:?}",
        rect.can_hold(&Rectangle { wight: 3, hight:4 })
    );


    println!(
        "rect can_hold_2 {:?}",
        rect.can_hold_2(&Rectangle { wight: 3, hight:4 })
    );

    let square = Rectangle::square(6);

    println!("{} {:#?}","square".green(), square);
    println!("square area {:?}", square.area());
}
