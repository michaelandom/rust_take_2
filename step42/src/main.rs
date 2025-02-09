use step42::Post;
use step42::Post2;


fn main() {

    let mut post = Post::new();
    let text= "i ate a pie for lunch today";
    post.add_text(text);
    assert_eq!("",post.content());


    post.request_review();
    assert_eq!("",post.content());


    post.approved();
    assert_eq!(text, post.content());



    let mut post = Post2::new();
    let text= "i ate a pie for lunch today";
    post.add_text(text);
    // assert_eq!("",post.content());


    let post =post.request_review();
    // assert_eq!("",post.content());


    let post=post.approve();
    assert_eq!(text, post.content());


}


