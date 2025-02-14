use std::ops::Add;

fn main() {

    
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