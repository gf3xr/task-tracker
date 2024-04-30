use std::{
    io::{self},
    time::{Duration, Instant},
    thread::sleep,
};

fn main() -> Result<(), io::Error> {
    loop {
        let mut task = String::new(); // Mutable strings to hold the users inputs
        let mut time_string = String::new();

        println!("Enter the task that you are doing?");
        io::stdin().read_line(&mut task)?;

        println!("Enter the time it will take (in minutes)?");
        io::stdin().read_line(&mut time_string)?;
        let time: u64 = match time_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for time. Please enter a valid number."); // checks if valid
                continue;
            }
        };

        println!("Task: {} you have {} minutes.", task.trim(), time);
        let amount_of_time = Duration::from_secs(time * 60);
        let start = Instant::now();

        while start.elapsed() < amount_of_time {
            sleep(Duration::from_secs(1)); // reduces cpu usage
        }
        println!("Time's up for: {}", task.trim());
    }
}
