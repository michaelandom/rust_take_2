use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s1 = String::from("s1");
    let s2 = "s2".to_string();

    let s3 = format!("{}{}", s1, s2);

    let s4 = s1 + &s2;

    for a in s4.chars() {
        println!("{}", a);
    }

    for a in s4.bytes() {
        println!("{}", a);
    }

    for a in s4.graphemes(true) {
        println!("{}", a);
    }

    let blue = "blue".to_string();
    let red = String::from("red");
    let yellow = String::from("yellow");

    let mut hash = HashMap::new();

    hash.insert(&blue, 20);
    hash.insert(&blue, 10);
    hash.insert(&red, 5);
    hash.insert(&red, 56);

    hash.entry(&yellow).or_insert(30);
    hash.entry(&yellow).or_insert(40);
    println!("hash {:#?}", hash);
    println!("hash {:#?}", blue);
    println!("hash {:#?}", red);

    let score = hash.get(&blue);

    match score {
        Some(num) => println!("some {num}"),
        None => println!("None {blue}"),
    }

    for (key, value) in &hash {
        println!("key {} : value {:?}", key, value);
    }


    count_string();
}


fn count_string() {

    let text = "hello world new world";


    let mut map= HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count+=1;
    } 


    print!("world count {:#?}", map);

}
