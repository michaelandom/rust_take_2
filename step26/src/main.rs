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

fn generate_workout(intensity: u32, random_number: i32) {
    let value = |num|  {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            value(intensity)
        );

        println!(
            "Next, do {} situps",
            value(intensity)
        );
    } else {
        if random_number ==3 {
            println!("Take a break Today")
        } else {
            println!(
                "Next, run for {} minutes!",
                value(intensity)
            );
        }
    }
}
