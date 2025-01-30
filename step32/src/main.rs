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

struct MyBox<T> (T);


impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}