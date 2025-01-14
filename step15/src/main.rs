
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




}
