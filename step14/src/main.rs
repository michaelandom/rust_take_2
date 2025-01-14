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




    enum SheetCell {
        Int(i32),
        Float(f32),
        Text(String)
    }


    let v2 = vec![SheetCell::Int(2),SheetCell::Int(2),SheetCell::Float(2.0),SheetCell::Text(String::from("as"))];

// v2.get(2)
    match v2.get(2) {
        Some(SheetCell::Int(num)) => println!("it is {num}"),
        _ => println!("not int") 
    }
    
}
