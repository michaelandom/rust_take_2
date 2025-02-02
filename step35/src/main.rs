use std::usize;

fn main() {
    // let a = 5;
    // let b = &mut a;

    // let mut c =10;
    // let d = &c;
    // println!("d {}",*d);
    // *d = 20; //cannot assign to `*d`, which is behind a `&` reference  `d` is a `&` reference, so the data it refers to cannot be written

    let a = LimitTracker{
        message: &Message {},
        max:10
    };

    

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

    #[test]
    fn it_sends_an_over_75(){
        let mock_message = MockMessage::new();
        let mut limitTracker = LimitTracker::new(&mock_message, 100);
        limitTracker.set_value(80);
        assert_eq!(mock_message.sent_messages.len(),1);


    }

}

struct MockMessage{
    sent_messages: Vec<String>
}

impl MockMessage {

    fn new() -> MockMessage{
        MockMessage {
            sent_messages: vec![]
        }
    }
}

impl Message for MockMessage {

    fn send(&mut self, msg: &str) {
        self.sent_messages.push(String::from(msg));
    }
    
}