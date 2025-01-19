fn main() {
    //    let r;

    //    {
    //     let x = 4;
    //     r = &x;
    //    }

    //     println!("Hello, world! {}",r);

    let string1 = String::from("here");
    let string2 = String::from("here");

    let result = longest(string1.as_str(), string2.as_str());
    println!("Hello, world! {}", result);
    let result2;
    {
    let string3 = String::from("hesssssss");

     result2 = longest(string1.as_str(), string3.as_str());

    }

    //`string3` does not live long enough 

    println!("Hello, world! {}", result2);
    

    let n =String::from("Call me you. as");
    let s= n.split(".").next().expect("error");

    let i =ImportantExcerpt {
        part: s
    };


    println!("Hello, world! {}", i.part);



}


struct  ImportantExcerpt<'a> {
    part: &'a str,
}


// we cant send back local var ref 
fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}
