fn main() {
    let mut s1 = String::from("Hello, world!");

    let word = first_word(&s1);
    // s1.clear(); b/c &mut and & 

    println!("word {}", word);

    s1.clear(); // this is okay 



    let s2 = "hello you";

    let word = first_word(&s2);

    println!("word {}", word);

}

fn first_word(s: &str) -> &str {
    let byte = s.as_bytes();
    for (index, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    return &s[..];
}
