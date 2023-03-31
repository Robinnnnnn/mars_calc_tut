use std::io;

fn main() {
    println!("Please input your weight in lbs as a float: (Ex. 100lbs = 100.0)");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight: f32 = calculate_weight_on_mars(weight);

    println!( "Weight on Mars:: {}lbs",mars_weight);

}


fn calculate_weight_on_mars(weight: f32) -> f32 {

    (weight/9.81) * 3.711

}

