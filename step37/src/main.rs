use std::{thread, time::Duration};

fn main() {
   let child = thread::spawn(|| {

        for i in 1..10{
            println!("Hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    child.join().unwrap();


    for i in 1..5{
        println!("Hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }

}
