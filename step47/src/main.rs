use std::fmt::{self, write};

fn main() {

    let wrapper=Wrapper(vec![String::from("a"),String::from("b")]);


    println!("wrapper {}", wrapper);
}


struct Wrapper(Vec<String>);



impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self.0.join(","))
    }
}

