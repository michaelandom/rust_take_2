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
    }

    
}
#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japanese
}