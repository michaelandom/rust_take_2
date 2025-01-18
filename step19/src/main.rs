fn main() {

    let newArticle = NewArticle{ author: String::from("aaa"), headline: String::from("a"), content: String::from("asa")};

    let tweet = Tweet {
        username:String::from("aaa"),
        content:String::from("aaa"),
        reply: false,
        retweet: false,
    };


    println!("newArticle {:?}",newArticle.summarize());
    println!("Tweet {:?}",tweet.summarize());


    println!("newArticle test {:?}",newArticle.test());
    println!("Tweet test {:?}",tweet.test());
    


    println!("Tweet notify {:?}",notify(&tweet));

    
}


pub struct NewArticle {
    author:String,
    headline:String,
    content:String
}

impl Summary for NewArticle {

    fn summarize(&self) -> String {
        format!("{}, by {}",self.content,self.author)
    }
    
}
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub struct Tweet{
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}


impl Summary for Tweet {

    fn summarize(&self) -> String {
        format!("{}, br {}", self.content,self.username)
    }
    
}


pub trait Summary {
    fn summarize(&self) -> String{
        String::from("Read more ...")
    }
    fn test(&self) -> String{
        String::from("Read more ...")
    }
}





