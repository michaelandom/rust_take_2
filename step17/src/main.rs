// use std::fs::File;
use std::fs::{self,File};
use std::io::{ErrorKind, Read};

pub struct Guess{
     value: i32
}

impl Guess {

    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100{
            panic!("Guess value must be 1 and 100 get {}",value);
        }
        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    
}

fn main() {

    let guess = Guess::new(-5);

    println!("guess {}", guess.value);

    method_1();
    method_2();
    // method_3();
    method_4();
    method_6();
    method_7();
}

fn method_1() {
    let f = File::open("t.txt");

    let f = match f {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("t.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem creating file: {:?}", other_error)
            }
        },
    };
}

fn method_2() {

    let f = File::open("t2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("t2.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem creating file: {:?}", error);
        }
    });
}

fn method_3(){

    let f = File::open("t3.txt").expect("Problem opening file");
}


fn method_4() -> Result<String, std::io::Error> {

    let  f = File::open("h4.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),  
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }

}

fn method_5() -> Result<String, std::io::Error> {
    let mut s = String::new();
    let mut f = File::open("h4.txt")?;

    f.read_to_string(&mut s)?;

    Ok(s)
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),  
    // };

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }
}

fn method_6() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("h4.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn method_7() -> Result<String,std::io::Error> {
    fs::read_to_string("h5.txt")
}