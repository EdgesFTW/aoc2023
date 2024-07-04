use std::{i32::MIN, sync::Arc, thread};

use regex::Regex;

#[allow(dead_code)]
pub fn day2_pt1(input: String) -> i64 {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut sum: i64 = 0;
    let arc_in = Arc::new(input);
    let lines = arc_in.lines();
    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();

    for line in lines {
        let Some(colon) = line.find(':') else {
            panic!("No colon in input line")
        };
        let id: i32 = line[5..colon].parse().unwrap();
        let mut valid_id = true;
        for (_, [q_str, color_str]) in re.captures_iter(line).map(|e| e.extract()) {
            let quantity: i32 = q_str.parse().unwrap();
            match color_str {
                "red" => {
                    if quantity > MAX_RED {
                        valid_id = false;
                        break;
                    }
                }
                "green" => {
                    if quantity > MAX_GREEN {
                        valid_id = false;
                        break;
                    }
                }
                "blue" => {
                    if quantity > MAX_BLUE {
                        valid_id = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if valid_id {
            sum += id as i64;
        }
    }

    return sum;
}

#[allow(dead_code)]
pub fn day2_pt2(input: String) -> i64 {
    const NUM_THREADS: usize = 7;
    let mut sum = 0;
    let lines = input.lines().collect::<Vec<_>>();
    let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();

    for line in lines {
        let mut max_red: i32 = MIN;
        let mut max_green: i32 = MIN;
        let mut max_blue: i32 = MIN;
        for (_, [q_str, color_str]) in re.captures_iter(line).map(|e| e.extract()) {
            let quantity: i32 = q_str.parse().unwrap();
            match color_str {
                "red" => {
                    if quantity > max_red {
                        max_red = quantity;
                    }
                }
                "green" => {
                    if quantity > max_green {
                        max_green = quantity;
                    }
                }
                "blue" => {
                    if quantity > max_blue {
                        max_blue = quantity;
                    }
                }
                _ => {
                    println!("Invalid color found somehow")
                }
            }
        }
        let power = max_red * max_blue * max_green;
        sum += power as i64;
    }
    return sum;
}

#[allow(dead_code)]
pub fn day2_pt2_parallel(input: String) -> i64 {
    const NUM_THREADS: usize = 7;
    let mut main_sum = 0;
    let lines = input.lines().collect::<Vec<_>>();
    let num_lines_per_thread = lines.len() / NUM_THREADS;
    let remainder = lines.len() - NUM_THREADS * num_lines_per_thread;

    thread::scope(|e| {
        let mut handles: Vec<thread::ScopedJoinHandle<i64>> = vec![];
        for i in 0..(NUM_THREADS + 1) {
            let lines = &lines;
            handles.push(e.spawn(move || {
                let mut sum = 0;
                let lower = i * num_lines_per_thread;
                let upper = if i == NUM_THREADS {
                    i * num_lines_per_thread + remainder
                } else {
                    (i + 1) * num_lines_per_thread
                };
                let re = Regex::new(r"([0-9]+) (red|green|blue)").unwrap();

                for line in &lines[lower..upper] {
                    let mut max_red: i32 = MIN;
                    let mut max_green: i32 = MIN;
                    let mut max_blue: i32 = MIN;
                    for (_, [q_str, color_str]) in re.captures_iter(line).map(|e| e.extract()) {
                        let quantity: i32 = q_str.parse().unwrap();
                        match color_str {
                            "red" => {
                                if quantity > max_red {
                                    max_red = quantity;
                                }
                            }
                            "green" => {
                                if quantity > max_green {
                                    max_green = quantity;
                                }
                            }
                            "blue" => {
                                if quantity > max_blue {
                                    max_blue = quantity;
                                }
                            }
                            _ => {
                                println!("Invalid color found somehow")
                            }
                        }
                    }
                    let power = max_red * max_blue * max_green;
                    sum += power as i64;
                }
                return sum;
            }));
        }
        for handle in handles {
            let val = handle.join().unwrap();
            main_sum += val;
        }
    });

    return main_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
        .to_string();
        assert!(day2_pt1(input) == 8);
    }
    #[test]
    fn test2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
        .to_string();
        assert!(day2_pt2(input) == 2286);
    }
}
