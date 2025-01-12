

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



     value_in_cents(Coin::Nickel(UsState::Alabama));


     
     let a = Some(5);
     let b = plus_one(a);
     let c = plus_one(None);


     println!(" a {:?}, b {:?}, c {:?}",a,b,c);


     let some_num = Some(4);

     match some_num {
        Some(3)=> println!("three"),
        Some(5)=> println!("five"),
        _ => ()
     }

     if let Some(3) = some_num {
        println!("there");
     } else if let Some(5) = some_num {
        println!("five");
     } else {
        ()
     }



}
#[derive(Debug)]
enum Coin {
    Penny(UsState),
    Nickel(UsState),
    Dime(UsState),
    Quarter(UsState)
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}

fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny(state) => {
            p("Penny",state);
            1
        },
        Coin::Nickel(state) => {
            p("Nickel",state);
            5
        },
        Coin::Dime(state) => {
            p("Dime",state);
            10},
        Coin::Quarter(state) => {
            p("Quarter",state);
            25
        },
    }
}

fn p(coin_name: &str, state: UsState){
    println!("state {:?} for {:?}!", coin_name,state);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(num) => Some(num+1),
        None => None  
    }
}