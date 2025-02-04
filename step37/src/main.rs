use std::{thread, time::Duration};

fn main() {

    let v = vec![1,2,4];

   let child = thread::spawn(move || {
        println!("v in {:?}",v);
    });

    child.join().unwrap();
    for i in 1..5{
        println!("Hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }

}
