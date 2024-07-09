use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 4,
        "Use the input filename, day and pt[1 2]as arguments"
    );
    let input = std::fs::read_to_string(&args[1]).unwrap();
    match (args[2].as_str(), args[3].as_str()) {
        ("day1", "pt1") => {
            let result1 = day1::day1_pt1(input);
            println!("Sum1: {}", result1);
        }
        ("day1", "pt2") => {
            let result2 = day1::day1_pt2_parallel(input);
            println!("Sum2: {}", result2);
        }
        ("day2", "pt1") => {
            let result1 = day2::day2_pt1(input);
            println!("Sum1: {}", result1);
        }
        ("day2", "pt2") => {
            let result2 = day2::day2_pt2_parallel(input);
            println!("Sum2: {}", result2);
        }
        ("day3", "pt1") => {
            let result1 = day3::day3_pt1(input);
            println!("Sum1: {}", result1);
        }
        ("day3", "pt2") => {
            let result2 = day3::day3_pt2(input);
            println!("Sum2: {}", result2);
        }
        ("day4", "pt1") => {
            let result1 = day4::day4_pt1(input);
            println!("Sum1: {}", result1);
        }
        ("day4", "pt2") => {
            let result2 = day4::day4_pt2(input);
            println!("Sum2: {}", result2);
        }
        _ => println!("Use day and part as args. eg day1 pt1"),
    }
}
