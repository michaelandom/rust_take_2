use std::sync::Mutex;

fn main() {

    let x = Mutex::new(5);


    {

        let mut num = x.lock().unwrap();

        *num = 6
    }


    println!("x in {:?}",x);

}
