use std::{rc::Rc, sync::{Arc, Mutex}, thread};

fn main() {

    let x = Mutex::new(5);


    {

        let mut num = x.lock().unwrap();

        *num = 6
    }


    println!("x in {:?}",x);


    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];


    for _ in 1..10  {
        let local = Arc::clone(&counter);
        let han = thread::spawn(move || {
            let mut count = local.lock().unwrap();
            *count+=1
        });
        handlers.push(han);
    }


    for h in handlers {
        h.join().unwrap();
    }


    println!("counter {:?}",counter);

}
