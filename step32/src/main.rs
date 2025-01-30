use std::ops::Deref;
fn main() {
    let x =3;
    let y = &x;

    assert_eq!(3,x);
    assert_eq!(3,*y);

    let y = Box::new(x);

    assert_eq!(3,*y);



    let y = MyBox::new(x);

    assert_eq!(3,*y);

}


impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0 
     }
}
struct MyBox<T> (T);


impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}