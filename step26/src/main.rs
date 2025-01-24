use std::{thread, time::Duration};

fn simulated_calc(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation:T) -> Cacher<T>{
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg:u32) -> u32 {

        match self.value {
            Some(v) => v,
            None => {

                let v = (self.calculation)(arg);
                self.value = Some(v);
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
