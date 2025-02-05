
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {


    let (tx,rx) = mpsc::channel();

    let tx2 = tx.clone();

     thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for msg in msgs{
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));  
        }
     });

     thread::spawn(move || {
        let msgs = vec![
            String::from("hi 2"),
            String::from("from 2"),
            String::from("the 2"),
            String::from("thread 2"),
        ];
        for msg in msgs{
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));  
        }
     });

     for receiver in rx {
        println!("Got: {}",receiver);       
     }

}
