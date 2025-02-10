use std::usize;

fn main() {

    let language= Language::English;
    match  language {

        Language::English => println!("English"),
        Language::Spanish => println!("Spanish"),
        Language::Russian => println!("Russian"),
        //Language::Japanese => println!("Japanese"),
        lang => println!("Other {:?}", lang),
        // _ => println!("Other"),
        
    }

    let authorization_status: Option<&str> = None;
    let is_admin= false;
    let group_id: Result<u8, _> = "34".parse();


    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin{
        println!("Authorization status admin");
    } else if let Ok(group_id) = group_id{

        if group_id > 30{
        println!("Authorization status privileged");
        } else {
            println!("Authorization status basic");
        }
    } else {
        println!("Authorization status guest");
    }


    let mut stack = Vec::new();


    stack.push(1);
    stack.push(3);
    stack.push(6);


    while let Some(top) = stack.pop() {
        println!("{}",top);
    }

    for (index,value) in stack.iter().enumerate(){
        println!("index {} value {}",index,value)
    }

    let (x,y,z) = (1,2,3.0);

    let point = (1,2);

    print_coord(&point);

    
}
#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japanese
}

fn print_coord(&(x,y): &(i32,i32)) {
    println!("Current location: ({}, {})",x,y)
}