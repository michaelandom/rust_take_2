use std::{collections::HashMap, hash::Hash, thread, time::Duration};

fn sim_calculating(ints: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    ints
}
fn main() {
    let intensity: u32 = 27;
    let random_number: u32 = 7;
    generate(intensity,random_number);
}

fn generate(intensity:u32,random_number:u32) {
    let mut call_sim_calculate = Cacher::new(|arg| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        arg 
    });

    if intensity == 27 {
        println!("Today, do {} pushups", call_sim_calculate.value(intensity));

        println!("Next, do {} situps", call_sim_calculate.value(intensity));
    } else {
        if random_number == 3 {
            println!("rest")
        } else {
        println!("Today, run for {} km", call_sim_calculate.value(intensity));
        }
    }

}

struct Cacher<T,U> 
where T : Fn(U) -> U,
      U : Copy + Eq + Hash
{

    calculate: T,
    values: HashMap<U,U>
}

impl<T,U>  Cacher<T,U>
where T : Fn(U) -> U,
      U : Copy + Eq + Hash
{

    fn new(calculate: T) -> Cacher<T,U> {
        Cacher{
            calculate,
            values: HashMap::new()
        }
    } 

    fn value(&mut self,args: U) -> U {
        match self.values.get(&args) {
            Some(num) => num.clone(),
            None => {
                let x = (self.calculate)(args);
                self.values.insert(args, x);
                x
            }
        }
    }
}