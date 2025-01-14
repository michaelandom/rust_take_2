
use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

fn main() {

    let s1 = String::from("s1");
    let s2 = "s2".to_string();
    
    let s3 = format!("{}{}",s1,s2);

    let s4 = s1 + &s2;


    for a in s4.chars(){
        println!("{}",a);
    }

    for a in s4.bytes(){
        println!("{}",a);
    }

    for a in s4.graphemes(true){
        println!("{}",a);
    }



    let blue = "blue".to_string();
    let red = String::from("red");


    let mut hash= HashMap::new();


    hash.insert(&blue, 10);
    hash.insert(&red, 5);
    println!("hash {:#?}",hash);
    println!("hash {:#?}",blue);
    println!("hash {:#?}",red);

    let score = hash.get(&blue);


    match score {
        Some(num) => println!("some {num}"),
        None => println!("None {blue}")
    }


}
