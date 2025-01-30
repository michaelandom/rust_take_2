fn main() {
    let x =3;
    let y = &x;

    assert_eq!(3,x);
    assert_eq!(3,*y);

    let y = Box::new(x);

    assert_eq!(3,*y);

}
