use core::panic;
use std::{collections::HashSet, usize};

#[allow(dead_code)]
pub fn day3_pt1(input: String) -> i64 {
    let arr: Vec<_> = input
        .lines()
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect();
    let line_width: usize = arr[0].len();
    let num_lines: usize = arr.len();
    let mut seen = HashSet::new();
    let mut sum = 0;
    let rows_range = 0..num_lines as i32;
    let col_range = 0..line_width as i32;
    let deltas: Vec<(i32, i32)> = (0..9).map(|e| (e / 3 - 1, e % 3 - 1)).collect();
    for i in 0..(num_lines as i32) {
        for j in 0..(line_width as i32) {
            assert!(arr[i as usize].len() == line_width);
            let cur_char = arr[i as usize][j as usize];
            // filter out non-symbols
            match cur_char {
                '.' => continue,
                '0'..='9' => continue,
                _ => (),
            }

            for (dx, dy) in &deltas {
                // check if index falls within grid
                if !rows_range.contains(&(dy + i)) || !col_range.contains(&(dx + j)) {
                    continue;
                }
                if !('0'..='9').contains(&arr[(i + dy) as usize][(j + dx) as usize]) {
                    continue;
                }
                let mut upperbound = (j + dx) as usize;
                for ind in ((j + dx + 1) as usize)..(line_width) {
                    if !('0'..='9').contains(&arr[(i + dy) as usize][ind]) {
                        upperbound = ind - 1;
                        break;
                    }
                    upperbound = line_width - 1;
                }
                let mut lowerbound = (j + dx) as usize;
                for ind in ((0)..=((j + dx - 1) as usize)).rev() {
                    if !('0'..='9').contains(&arr[(i + dy) as usize][ind]) {
                        lowerbound = ind + 1;
                        break;
                    }
                    lowerbound = 0;
                }
                match arr[(i + dy) as usize][lowerbound..=upperbound]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                {
                    Ok(x) => {
                        if seen.contains(&(i + dy, lowerbound, upperbound)) {
                        } else {
                            // unique id is (most sig row, most sig col)
                            seen.insert((i + dy, lowerbound, upperbound));
                            sum += x;
                        }
                    }
                    Err(x) => println!("Internal Error: {x}"),
                }
            }
        }
    }
    sum.into()
}

#[allow(dead_code)]
pub fn day3_pt2(input: String) -> i64 {
    let arr: Vec<_> = input
        .lines()
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect();
    let line_width: usize = arr[0].len();
    let num_lines: usize = arr.len();
    let mut sum = 0;
    for i in 0..(num_lines as i32) {
        for j in 0..(line_width as i32) {
            assert!(arr[i as usize].len() == line_width);
            let cur_char = arr[i as usize][j as usize];
            match cur_char {
                '.' => continue,
                '0'..='9' => continue,
                '*' => {
                    let mut seen: HashSet<_> = HashSet::new();
                    let mut first = None;
                    let mut second = None;
                    let mut third = None;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            match (dx, dy) {
                                (0, 0) => continue,
                                (dx, _) if (j + dx < 0) || (j + dx >= (line_width as i32)) => {
                                    continue;
                                }
                                (_, dy) if (i + dy < 0) || (i + dy >= (num_lines as i32)) => {
                                    continue;
                                }
                                (dx, dy)
                                    if (j + dx >= 0)
                                        && (j + dx < (line_width as i32))
                                        && (i + dy >= 0)
                                        && (i + dy < (num_lines as i32)) =>
                                // outer box ind within bounds of grid
                                {
                                    if !('0'..='9')
                                        .contains(&arr[(i + dy) as usize][(j + dx) as usize])
                                    {
                                        continue;
                                    }
                                    let mut upperbound = (j + dx) as usize;
                                    for ind in ((j + dx + 1) as usize)..(line_width) {
                                        if !('0'..='9').contains(&arr[(i + dy) as usize][ind]) {
                                            upperbound = ind - 1;
                                            break;
                                        }
                                        upperbound = line_width - 1;
                                    }
                                    let mut lowerbound = (j + dx) as usize;
                                    for ind in ((0)..=((j + dx - 1) as usize)).rev() {
                                        if !('0'..='9').contains(&arr[(i + dy) as usize][ind]) {
                                            lowerbound = ind + 1;
                                            break;
                                        }
                                        lowerbound = 0;
                                    }
                                    match arr[(i + dy) as usize][lowerbound..=upperbound]
                                        .iter()
                                        .collect::<String>()
                                        .parse::<u32>()
                                    {
                                        Ok(x) => match (first, second) {
                                            (None, None) => {
                                                seen.insert((i + dy, lowerbound, upperbound));
                                                first = Some(x);
                                            }
                                            (Some(_), None) => {
                                                if !seen.contains(&(i + dy, lowerbound, upperbound))
                                                {
                                                    seen.insert((i + dy, lowerbound, upperbound));
                                                    second = Some(x);
                                                }
                                            }
                                            (Some(_), Some(_)) => {
                                                if !seen.contains(&(i + dy, lowerbound, upperbound))
                                                {
                                                    seen.insert((i + dy, lowerbound, upperbound));
                                                    third = Some(x);
                                                }
                                            }
                                            (None, Some(_)) => {
                                                panic!("invalid state! {:?}", (first, second));
                                            }
                                        },
                                        Err(x) => println!("Error: {x}"),
                                    }
                                }
                                (dx, dy) => panic!("Unknown indexing. dx: {dx} dy: {dy}"),
                            }
                        }
                    }

                    match (first, second, third) {
                        (Some(first), Some(second), None) => {
                            sum += first * second;
                        }
                        (_, _, _) => (),
                    }
                }
                _ => (),
            }
        }
    }
    sum.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        .to_string();
        let result = day3_pt1(input.clone());
        println!("{input}");
        println!("Result: {result}");
        assert!(result == 4361);
    }
    #[test]
    fn test2() {
        let input = "467..114..
...*......
"
        .to_string();
        let result = day3_pt1(input.clone());
        println!("{input}");
        println!("Result: {result}");
        assert!(result == 467);
    }

    #[test]
    fn test3() {
        let input = "....*.....
467..114..
....*.....
"
        .to_string();
        let result = day3_pt1(input.clone());
        println!("{input}");
        println!("Result: {result}");
        assert!(result == 114);
    }

    #[test]
    fn test4() {
        let input = "....*...
467..114
"
        .to_string();
        let result = day3_pt1(input.clone());
        println!("{input}");
        println!("Result: {result}");
        assert!(result == 114);
    }

    #[test]
    fn test5() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        .to_string();
        let result = day3_pt2(input.clone());
        println!("{input}");
        println!("Result: {result}");
        assert!(result == 467835);
    }
}
