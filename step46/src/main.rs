fn main() {

    
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