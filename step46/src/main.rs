use std::ops::Add;

fn main() {

    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    Human::fly2();
    <Human as Wizard>::fly2();
    <Human as Pilot>::fly2();

    
}


trait Pilot {
    fn fly(&self);
    fn fly2();
}

trait Wizard {
    fn fly(&self);
    fn fly2();
}


struct Human;

impl Human {
    fn fly(&self){
        println!("Human fly");
    }
    fn fly2(){
        println!("Human fly2");
    }
}

impl Wizard for Human {

    fn fly(&self) {
        println!("Wizard fly");
    }
    fn fly2(){
        println!("Wizard fly2");
    }
}


impl Pilot for Human {

    fn fly(&self) {
        println!("Pilot fly");
    }

    fn fly2(){
        println!("Pilot fly2");
    }
    
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;


    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
    
}


struct Point{
    x:i32,
    y:i32,
}


impl Add for Point{

    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter{}

impl Iterator<u32> for Counter {

    fn next(&mut self) -> Option<u32> {
        Some(1)
    }
}


impl Iterator<u16> for Counter {

    fn next(&mut self) -> Option<u16> {
        Some(1)
    }
}