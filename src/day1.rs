pub fn sum_calibration_pt1(input: String) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let mut first = -1;
        let mut last = -1;
        for char in line.chars() {
            if char.is_numeric() {
                let val = char.to_digit(10).unwrap();
                if first == -1 {
                    first = val as i32
                } else {
                    last = val as i32
                }
            }
        }
        if last == -1 {
            last = first;
        }
        sum += (10 * first + last) as i64;
    }
    return sum;
}

pub fn sum_calibration_pt2(input: String) -> i64 {
    let mut sum: i64 = 0;
    let lines = input.lines();
    // let lines_per_thread = lines.clone().count() / 4;
    let lines_vec: Vec<&str> = lines.collect();
    for line in lines_vec {
        let mut first = -1;
        let mut last = -1;
        for i in 0..line.len() {
            let offset_line = &line[i..line.len()];
            if offset_line.starts_with("1") {
                if first == -1 {
                    first = 1
                } else {
                    last = 1
                }
                continue;
            }
            if offset_line.starts_with("2") {
                if first == -1 {
                    first = 2
                } else {
                    last = 2
                }
                continue;
            }
            if offset_line.starts_with("3") {
                if first == -1 {
                    first = 3
                } else {
                    last = 3
                }
                continue;
            }
            if offset_line.starts_with("4") {
                if first == -1 {
                    first = 4
                } else {
                    last = 4
                }
                continue;
            }
            if offset_line.starts_with("5") {
                if first == -1 {
                    first = 5
                } else {
                    last = 5
                }
                continue;
            }
            if offset_line.starts_with("6") {
                if first == -1 {
                    first = 6
                } else {
                    last = 6
                }
                continue;
            }
            if offset_line.starts_with("7") {
                if first == -1 {
                    first = 7
                } else {
                    last = 7
                }
                continue;
            }
            if offset_line.starts_with("8") {
                if first == -1 {
                    first = 8
                } else {
                    last = 8
                }
                continue;
            }
            if offset_line.starts_with("9") {
                if first == -1 {
                    first = 9
                } else {
                    last = 9
                }
                continue;
            }
            if offset_line.starts_with("one") {
                if first == -1 {
                    first = 1
                } else {
                    last = 1
                }
                continue;
            }
            if offset_line.starts_with("two") {
                if first == -1 {
                    first = 2
                } else {
                    last = 2
                }
                continue;
            }
            if offset_line.starts_with("three") {
                if first == -1 {
                    first = 3
                } else {
                    last = 3
                }
                continue;
            }
            if offset_line.starts_with("four") {
                if first == -1 {
                    first = 4
                } else {
                    last = 4
                }
                continue;
            }
            if offset_line.starts_with("five") {
                if first == -1 {
                    first = 5
                } else {
                    last = 5
                }
                continue;
            }
            if offset_line.starts_with("six") {
                if first == -1 {
                    first = 6
                } else {
                    last = 6
                }
                continue;
            }
            if offset_line.starts_with("seven") {
                if first == -1 {
                    first = 7
                } else {
                    last = 7
                }
                continue;
            }
            if offset_line.starts_with("eight") {
                if first == -1 {
                    first = 8
                } else {
                    last = 8
                }
                continue;
            }
            if offset_line.starts_with("nine") {
                if first == -1 {
                    first = 9
                } else {
                    last = 9
                }
                continue;
            }
        }
        if last == -1 {
            last = first
        }
        sum += (10 * first + last) as i64;
        // println!("{}, {}", line, 10 * first + last);
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::{day1::sum_calibration_pt1, day1::sum_calibration_pt2};

    #[test]
    fn test1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert!(sum_calibration_pt1(input.to_owned()) == 142)
    }

    #[test]
    fn test2() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert!(sum_calibration_pt2(input.to_owned()) == 142)
    }

    #[test]
    fn test3() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        assert!(sum_calibration_pt2(input.to_owned()) == 281)
    }

    #[test]
    fn test4() {
        let input = "threefourtwone";
        assert!(sum_calibration_pt2(input.to_owned()) == 31)
    }
}