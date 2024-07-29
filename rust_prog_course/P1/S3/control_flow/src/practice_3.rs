// Problem 3:

/*
This question involves writing code to analyze the production of an assembly line in a car factory.
The assembly line has different speeds, ranging from 0 (off) to 10 (maximum).
At the lowest speed of 1, the assembly line produces a total of 221 cars per hour.
The production rate increases linearly with the speed,
meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

However, higher speeds increase the likelihood of producing faulty cars that need to be discarded.
The success rate depends on the speed, as shown in the table below:
· Speeds 1 to 4: 100% success rate.
· Speeds 5 to 8: 90% success rate.
· Speeds 9 and 10: 77% success rate.

You need to write two functions:
1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours.
    The function takes the number of hours and speed as input and returns the number of cars successfully produced.
2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute.
    The function takes the number of hours and speed as input and returns the number of cars produced per minute.
Write the code for both functions based on the provided specifications.
*/

fn main() {
    println!("Main program to display results.");
    println!("{}", total_production(6, 5) as f32);

}

fn total_production(hours: u16, speed: u16) -> f32 {
    // let success_rate: f32;
    // production capacity 221 cars per hour stat.
    // assembly line 0 is off with max speed 10
    // 2 hours, 4 speed = 2 * 4 * 221
    // total_produced_cars logic
    let success_rate: f32 = match speed {
        1..=4 => 1f32,
        5..=8 => 0.9f32,
        9..=10 => 0.77f32,
        _ => 0f32,
    };

    let result = (hours * speed * 221 ) as f32 * success_rate;
    result
}

fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
    let success_rate: f32;
    -1.0
}