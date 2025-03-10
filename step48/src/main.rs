fn add_one(num: i32) -> i32 {
    num + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("the ans is : {}", answer);

    let list_num = vec![1, 2];

    let list_string: Vec<String> = list_num.iter().map(|i| i.to_string()).collect();

    println!("{:?}", list_string);

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn returns_closure(arg: i32) -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_closure_2(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}
