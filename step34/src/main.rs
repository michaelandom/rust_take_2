fn main() {

    let a = Cons(10,Box::new(Cons(5,Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(3, Box::new(a));
}


enum List {
    Cons(i32,Box<List>),
    Nil
}

use crate::List::{Cons,Nil};