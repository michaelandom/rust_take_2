struct Post {
    state: Box<dyn State>,
    contant: String
}

impl Post {

    fn new() -> Post {
        Post {
            state: Box::new(Draft {}),
            contant: String::new()
        }
    }
    

    pub fn add_text(&mut self, value: &str) {
        self.contant.push_str(value);
    }
}

trait State {
    
}


struct Draft{

}

impl State for Draft {
}


