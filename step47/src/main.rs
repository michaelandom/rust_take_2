use std::fmt::{self, write};

fn main() {

    let wrapper=Wrapper(vec![String::from("a"),String::from("b")]);


    println!("wrapper {}", wrapper);

    type  kelo = i32;


    let a =5;
    let b: kelo= 10;

    println!("a + b = {}",a+b);

    type lont_type = Box<dyn Fn() + Send + 'static>;

    let f: lont_type = Box::new( || println!("hi"));

    fn take_long_type(f: lont_type) {

    }

    fn take_long_type_2(f: lont_type) {

    }

    let s1: &str = "herl";


}


struct Wrapper(Vec<String>);



impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]",self.0.join(","))
    }
}

