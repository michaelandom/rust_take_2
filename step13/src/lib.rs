use rand::{Rng,CryptoRng,ErrorKind::Transient};


// use std:io;
use std::io::Write;
use std::io::{self,Read};

mod front_of_house;
 
mod back_of_house{
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
         seasonal_fruit: String
    }

    impl Breakfast {

       pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Apple")
            }
        }
        
    }
    fn fix_incorrect_order(){
        cook_oder();
        super::front_of_house::serving::serve_order();
    }

    fn cook_oder(){}

}

use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
pub fn eat_at_restaurant() {
    // use path
    hosting::add_to_waitlist();
   
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("AAA");


    meal.toast = String::from("aa");

    println!("meal {:#?}",meal);
}



