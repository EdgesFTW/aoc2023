use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(
        args.len() == 3,
        "Use the input filename and pt[1 2]as arguments"
    );
    let input = std::fs::read_to_string(&args[1]).unwrap();
    match args[2].as_str() {
        "pt1" => {
            let result1 = day4::day4_pt1(input);
            println!("Sum1: {}", result1);
        }
        "pt2" => {
            let result2 = day4::day4_pt2(input);
            println!("Sum2: {}", result2);
        }
        _ => println!("Use pt1 or pt2 as the last arg"),
    }
}
