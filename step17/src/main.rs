use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    method_1();
    method_2();
    // method_3();
    method_4();
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