struct Post {
    state: Option<Box<dyn State>>,
    contant: String
}

impl Post {

    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            contant: String::new()
        }
    }
    
    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_text(&mut self, value: &str) {
        self.contant.push_str(value);
    }

    pub fn request_review(&mut self){

        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approved(&mut self){
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

trait State {

    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post:&'a Post) -> &'a str {
        ""
    }
    
}


struct Draft{

}

impl State for Draft {

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    

}

struct PendingReview {
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    
}

struct Published{
}

impl State for Published {

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post:&'a Post) -> &'a str {
         &post.contant
    }
}