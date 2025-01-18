use std::fmt::Display;


struct Pair<T> {
    x:T,
    y:T,
}



impl<T>  Pair<T> {
    fn new(x:T, y:T) -> Pair<T> {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
    
}



fn main() {

    let pair = Pair::new(1, 2);


    println!("pair is {:?}", pair.cmp_display());



}
