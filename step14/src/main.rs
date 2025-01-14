fn main() {
    let x = [1,2,3];
    let mut v1= vec![1,2,4];

    v1.push(5);
    

    match v1.get(2) {
        Some(num) => println!("num is {}",num),
        None => println!("num is None")
    }

    for index in &mut v1 {
        *index += 50;
        println!("at {index}");
    }
    for (index , value) in v1.iter().enumerate(){
        println!("at {index} is {}",value);
    }

    
}
