
fn add_one(num: i32)  -> i32{
    num +1
}

fn do_twice<T>(f:T,arg:i32 ) -> i32
where T : Fn(i32) -> i32
{
    f(arg) + f(arg)
}


fn main() {
    let answer = do_twice(add_one, 5);
    println!("the ans is : {}", answer);
}



