use std::time::Duration;
use std::thread;
fn main() {
    let simulated_user_specified_value=10;
    let simulated_rand_num=7;
    generate_workout(simulated_user_specified_value,simulated_rand_num);
}

fn simulated_expensive_calculation(intensity:u32)->u32{
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn generate_workout(intensity:u32,random_number:u32){
    if intensity < 25{
        println!("today do {} pushups",simulated_expensive_calculation(intensity));

        println!("next do {} situps",simulated_expensive_calculation(intensity));
    }
}