fn main() {
 let v = vec!["1","b"];
 let v1 = vec![1,2,3];  
}


#[macro_export]
macro_rules! vec {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}