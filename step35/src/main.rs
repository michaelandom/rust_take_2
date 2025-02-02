use std::{cell::RefCell, rc::Rc, usize};
use crate::List::{Cons,Nil};

#[derive(Debug)]
enum List {
    Cons (Rc<RefCell<i32>>, Rc<List>),
    Nil
}
fn main() {

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));


    *value.borrow_mut() +=10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);


   
   }



pub trait Message {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Message> {
    message: &'a T,
    value: usize,
    max: usize,
}


impl<'a, T> LimitTracker<'a,T> 
where T : Message
{
    
    fn new(message: &T, max:usize) -> LimitTracker<T> {
        LimitTracker { message, value: 0, max }
    }


    fn set_value(&mut self,value:usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;


        if percentage_of_max >= 1.0 {
            self.message.send("Error: you are over your plan");
        } else if percentage_of_max >= 0.9 {
            self.message.send("Urgent warning: you've used up over 90% of you plan");
        } else if percentage_of_max >= 0.75 {
            self.message.send("warning: you have used up over 75% of your plan");
        }
    }

}


#[cfg(test)]
mod message_test{
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn it_sends_an_over_75(){
        let mock_message = MockMessage::new();
        let mut limitTracker = LimitTracker::new(&mock_message, 100);
        limitTracker.set_value(80);
        assert_eq!(mock_message.sent_messages.borrow().len(),1);
    }

    struct MockMessage{
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessage {
    
        fn new() -> MockMessage{
            MockMessage {
                sent_messages: RefCell::new(vec![])
            }
        }
    }
    
    impl Message for MockMessage {
    
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg))
        }
        
    }

}