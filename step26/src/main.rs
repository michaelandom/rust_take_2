use std::{collections::HashMap, fmt::Display, hash::Hash, thread, time::Duration};

fn simulated_calc(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);


    let x =vec![1,2,3];
    let eq_to_x = move |z| z ==x; // fn cant get x but closures 
   
    // println!("cant get x {}", x);
   
    let y =vec![1,2,3];

    assert!(eq_to_x(y));

    //FnOnce, FnMut, Fn
}

struct Cacher<T,U>
where
    U: Copy + Eq + Hash,
    T: Fn(U) -> U
    {
    calculation: T,
    values: HashMap<U,U>,
}

impl<T,U> Cacher<T,U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash
{
    fn new(calculation:T) -> Cacher<T,U>{
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg:U) -> U{

        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v 
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: i32) {
    let mut cacher_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    if intensity < 25 {
        println!("Today, do {} pushups", cacher_result.value(intensity));

        println!("Next, do {} situps", cacher_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break Today")
        } else {
            println!("Next, run for {} minutes!", cacher_result.value(intensity));
        }
    }
}
