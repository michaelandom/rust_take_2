use step42::Post;

fn main() {

    let mut post = Post::new();
    let text= "i ate a pie for lunch today";
    post.add_text(text);
    assert_eq!("",post.content());


    post.request_review();
    assert_eq!("",post.content());


    post.approved();
    assert_eq!(text, post.content());


}


