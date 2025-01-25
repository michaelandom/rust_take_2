fn main() {

    let v1 = vec![1,2,3];

    let v1_iter = v1.iter(); // lazey


    for value in v1_iter {
        println!("Got: {}", value)
    }

}


#[test]
fn iterator_test_into(){
    let v1 = vec![1,2,3];

    // let mut v1_iter = v1.iter_mut();
    // let mut v1_iter = v1.iter();
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(),Some( 1));
    assert_eq!(v1_iter.next(),Some( 2));
    assert_eq!(v1_iter.next(),Some( 3));
    assert_eq!(v1_iter.next(),None);
}

#[test]
fn iterator_test_mut(){
    let mut v1 = vec![1,2,3];

    // let mut v1_iter = v1.iter_mut();
    // let mut v1_iter = v1.iter();
    let mut v1_iter = v1.iter_mut();

    assert_eq!(v1_iter.next(),Some(&mut 1));
    assert_eq!(v1_iter.next(),Some(&mut 2));
    assert_eq!(v1_iter.next(),Some(&mut 3));
    assert_eq!(v1_iter.next(),None);
}

#[test]
fn iterator_test(){
    let mut v1 = vec![1,2,3];

    // let mut v1_iter = v1.iter_mut();
    // let mut v1_iter = v1.iter();
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(),Some(& 1));
    assert_eq!(v1_iter.next(),Some(& 2));
    assert_eq!(v1_iter.next(),Some(& 3));
    assert_eq!(v1_iter.next(),None);
}