use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons,Nil};
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a instal {}",Rc::strong_count(&a));
    println!("a next tail {:?}",a.tail());



    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a instal {}",Rc::strong_count(&a));
    println!("b instal {}",Rc::strong_count(&b));
    println!("b next tail {:?}",b.tail());



    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }


    println!("a rc instal {}",Rc::strong_count(&a));
    println!("b rc instal {}",Rc::strong_count(&b));



    println!("a {:?}", a.tail());


}

#[derive(Debug)]
enum List {

    Cons(i32,RefCell<Rc<List>>),
    Nil
}


impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_,item) => Some(item),
            Nil => None
        }
    }
}