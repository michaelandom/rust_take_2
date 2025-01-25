fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // lazey

    for value in v1_iter {
        println!("Got: {}", value)
    }
}

#[derive(PartialEq,Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[test]
fn iterator_test_into() {
    let v1 = vec![1, 2, 3];

    // let mut v1_iter = v1.iter_mut();
    // let mut v1_iter = v1.iter();
    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_test_mut() {
    let mut v1 = vec![1, 2, 3];

    // let mut v1_iter = v1.iter_mut();
    // let mut v1_iter = v1.iter();
    let mut v1_iter = v1.iter_mut();

    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_test() {
    let mut v1 = vec![1, 2, 3];

    // let mut v1_iter = v1.iter_mut();
    // let mut v1_iter = v1.iter();
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    let sum: i32 = v1_iter.sum();

    assert_eq!(sum, 6);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, [2, 3, 4]);
}

#[test]
fn shoe_in_my_size_test() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("a")
        },
        Shoe {
            size: 15,
            style: String::from("b")
        },
        Shoe {
            size: 15,
            style: String::from("b")
        },
    ];

    let v2 = shoe_in_my_size(shoes, 10);
    let ans = vec![Shoe {
        size: 10,
        style: String::from("a")
    }];

    assert_eq!(ans, v2)
}
