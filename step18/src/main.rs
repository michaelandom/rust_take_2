fn main() {
    let v = vec![1, 2, 3, 5];

    let largest = get_largest(v);

    println!("The largest num {}", largest);

    let v = vec!['1', '2', '3', '5'];

    let largest = get_largest(v);

    println!("The largest char {}", largest);

    let points = Points { x: 2, y: 5.3 };

    println!("The points char {:?}", points);

    let points2 = Points { x: "Ss", y: "Sss" };

    println!("The points2 char {:?}", points2.mix_up(points));
}

fn get_largest<T: PartialOrd + Copy>(v: Vec<T>) -> T {
    let mut l = v[0];
    for num in v {
        if num > l {
            l = num;
        }
    }
    l
}
#[derive(Debug)]
struct Points<T, U> {
    x: T,
    y: U,
}

impl<T, U> Points<T, U> {
    fn mix_up<V,W>(self, other: Points<V, W>) -> Points<T, W> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}
