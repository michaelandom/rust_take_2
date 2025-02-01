fn main() {

    let a = Rc::new(Cons(10,Rc::new(Cons(5,Rc::new(Nil)))));
    let b = Cons(4, Rc::clone(&a));
    let c = Cons(2, Rc::clone(&a));
}


enum List {
    Cons(i32,Rc<List>),
    Nil
}

use std::rc::Rc;

use crate::List::{Cons,Nil};