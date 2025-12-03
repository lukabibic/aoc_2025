advent_of_code::solution!(3);

fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().filter(|l| !l.is_empty()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let battery_banks: Vec<&str> = parse_lines(input);
    let mut sum_of_combined_max: u64 = 0;

    for bank in battery_banks {
        // println!("bank = {}", bank);
        // get max value between index 0 and n-1
        let digits: Vec<u64> = bank
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect();
        if digits.is_empty() {
            println!("Bank {} contains no digits", bank);
            continue;
        }
        // Max number in digits and the its position from index 0 to n-1
        let max: &u64 = digits[..digits.len() - 1].iter().max().unwrap();
        // println!("Max value in bank {}: {}", bank, max);
        // first appearance of max
        let pos = digits.iter().position(|&x| x == *max).unwrap();

        // println!("Bank {}: Max value {} at position {}", bank, max, pos);
        // println!("Position of max value in bank {}: {}", bank, pos);
        // println!("Max value in bank {}: {}", bank, max);

        // find next max number after max position
        let next_max: Option<&u64> = digits.iter().skip(pos + 1).max();

        // concat max and next_max and convert to u64
        if let Some(next_max) = next_max {
            let combined = format!("{}{}", max, next_max);
            let combined_num: u64 = combined.parse().unwrap();
            sum_of_combined_max += combined_num;
            // println!("Combined max and next max in bank {}: {}", bank, combined_num);
        } else {
            println!("No next max value found in bank {}", bank);
        }
    }

    Some(sum_of_combined_max)
}

pub fn part_two(input: &str) -> Option<u64> {
    let battery_banks: Vec<&str> = parse_lines(input);
    let mut sum_of_combined_max: u64 = 0;
    // println!("hello");
    for bank in battery_banks {
        // println!("bank = {}", bank);
        // get max value between index 0 and n-1
        let digits: Vec<u64> = bank
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u64))
            .collect();
        if digits.is_empty() {
            // println!("Bank {} contains no digits", bank);
            continue;
        }
        // Max number in digits and the its position from index 0 to n-1
        let max: &u64 = digits[..digits.len() - 11].iter().max().unwrap();

        let mut max_string = max.to_string();
        // println!("Max value in bank {}: {}", bank, max);
        // first appearance of max
        let pos = digits.iter().position(|&x| x == *max).unwrap();

        // println!("Bank {}: Max value {} at position {}", bank, max, pos);

        let mut next_array = digits[pos + 1..].to_vec();

        for i in (0..11).rev() {
            // println!("i = {}", i);
            let next_max = next_array[..next_array.len() - i].iter().max().unwrap();
            let next_max_pos = next_array.iter().position(|&x| x == *next_max).unwrap();

            //concat max_str and next_max
            max_string += &next_max.to_string();
            println!("{}", max_string);
            next_array = next_array[next_max_pos + 1..].to_vec();
        }
        sum_of_combined_max += max_string.parse::<u64>().unwrap();
    }
    Some(sum_of_combined_max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
