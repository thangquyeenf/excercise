enum TrafficLight {
    Red,
    Yellow,
    Blue,
}

fn time_to_wait(light: TrafficLight) -> u32 {
    match light {
        TrafficLight::Red => 30,
        TrafficLight::Yellow => 5,
        TrafficLight::Blue => 0,
    }
}
fn main() {
    let red_light = TrafficLight::Red;
    println!("Time to wait for red light: {}s", time_to_wait(red_light));
    let yellow_light = TrafficLight::Yellow;
    println!(
        "Time to wait for yellow light: {}s",
        time_to_wait(yellow_light)
    );
    let blue_light = TrafficLight::Blue;
    println!("Time to wait for blue light: {}s", time_to_wait(blue_light));
}
