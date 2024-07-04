#[allow(dead_code)]
pub fn pt1(input: String) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    let mut sum = 0;
    for line in lines {
        let start_ind = line.find(':').expect("input line need a semicolon");
        let mid_ind = line.find('|').expect("input line need a pipe");
        let winning: Vec<_> = line[start_ind + 1..mid_ind]
            .split_whitespace()
            .map(|e| {
                e.parse::<u32>()
                    .expect("integers seperated by whitespace in input")
            })
            .collect();
        let your_nums: Vec<_> = line[mid_ind + 1..]
            .split_whitespace()
            .map(|e| {
                e.parse::<u32>()
                    .expect("integers seperated by whitespace in input")
            })
            .collect();
        let mut score = None;
        for num in your_nums {
            if winning.contains(&num) {
                match score {
                    None => score = Some(1),
                    Some(x) => score = Some(2 * x),
                }
            }
        }
        if let Some(x) = score {
            sum += x;
        }
    }

    sum
}

#[allow(dead_code)]
pub fn pt2(input: String) -> i64 {
    let lines: Vec<_> = input.lines().collect();
    let mut copies: Vec<u32> = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let start_ind = line.find(':').expect("input line need a semicolon");
        let mid_ind = line.find('|').expect("input line need a pipe");
        let winning: Vec<_> = line[start_ind + 1..mid_ind]
            .split_whitespace()
            .map(|e| {
                e.parse::<u32>()
                    .expect("integers seperated by whitespace in input")
            })
            .collect();
        let your_nums: Vec<_> = line[mid_ind + 1..]
            .split_whitespace()
            .map(|e| {
                e.parse::<u32>()
                    .expect("integers seperated by whitespace in input")
            })
            .collect();
        let mut wins = 0;
        for num in your_nums {
            if winning.contains(&num) {
                wins += 1;
            }
        }
        for delta in 1..=wins {
            copies[i + delta as usize] += copies[i];
        }
    }

    // println!("copies: {:?}", copies);

    copies.iter().sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .to_string();
        let result = pt1(input.clone());
        println!("{input}");
        println!("Result: {result}");
        assert!(result == 13);
    }

    #[test]
    fn test2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .to_string();
        let result = pt2(input.clone());
        println!("{input}");
        println!("Result: {result}");
        assert!(result == 30);
    }
}
