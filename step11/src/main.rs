

enum  IPKind {
    v4(String),
    v6
}

struct  IPAddress{
    kind: IPKind,
    address: String
}
fn main() {
    
    let ip4= IPKind::v4(String::from("127.0.0.1"));


    let ip6 = IPAddress{
        kind:IPKind::v6,
        address:String::from("127,0,0,1")
    };




    let x =3;
    let y = Some(5);


     let sum = x + match y {
            Some(num) => num,
            _ => 0
     };


     println!("sum {sum}");

     let sum = x + y.unwrap_or(0);

     println!("sum {sum}");







}
