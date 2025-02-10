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
    
}
#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japanese
}