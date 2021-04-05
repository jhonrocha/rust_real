use std::io;
fn main() {
    let mut input = String::new();
    println!("Mass on Earth:");
    io::stdin().read_line(&mut input).unwrap();
    let input: f32 = input.trim().parse().unwrap();
    dbg!(input);
    let mars_weight = calculate_weight_on_mars(input);
    println!("Weight on Mars {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
