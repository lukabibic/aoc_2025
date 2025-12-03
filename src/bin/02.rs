advent_of_code::solution!(2);


pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.trim().split(",").collect();
    println!("{:?}", ranges);

    let mut sum_of_repeating_number:u64 = 0;

    for range in &ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let start: &str = bounds[0];
        let end: &str = bounds[1];

        let start_num: u64 = start.parse().unwrap();
        let end_num: u64 = end.parse().unwrap();

        for i in start_num..=end_num {
            // if number is odd, skip
            if i.to_string().len() % 2 != 0 {
                continue;
            }
            // print!("{} ", i);
            // check if first half == second half
            let i_str = i.to_string();
            let length = i_str.len();
            let first_half = &i_str[..length / 2];
            let second_half = &i_str[length / 2..];
            if first_half == second_half {
                println!("Found matching number: {}", i);
                sum_of_repeating_number += i;
        }
        // println!("start = {}, end = {}", start, end);
    }

    }

    println!("Sum of repeating numbers: {}", sum_of_repeating_number);
    Some(sum_of_repeating_number)
}

pub fn part_two(input: &str) -> Option<u64> {

    let ranges: Vec<&str> = input.trim().split(",").collect();
    println!("{:?}", ranges);

    let mut sum_of_repeating_number:u64 = 0;

    for range in &ranges {
        let bounds: Vec<&str> = range.split("-").collect();
        let start: &str = bounds[0];
        let end: &str = bounds[1];

        let start_num: u64 = start.parse().unwrap();
        let end_num: u64 = end.parse().unwrap();

        for i in start_num..=end_num {
            let length_i = i.to_string().len();

            let mut sequence_len = 1;

            for j in 1..length_i {
                if length_i % j != 0 {
                    continue;
                }
                let segment = &i.to_string()[0..j];
                // split string by segment length
                let mut is_repeating = true;
                // check if all segments are equal to segment
                for k in 0..(length_i / j) {
                    let part = &i.to_string()[k * j..(k + 1) * j];
                    if part != segment {
                        is_repeating = false;
                        break;
                    }
                }
                if is_repeating {
                    sequence_len = length_i / j;
                    break;
                }
            }
            if sequence_len > 1 {
                sum_of_repeating_number += i;
            }
        }
    }

    println!("Sum of repeating numbers: {}", sum_of_repeating_number);
    Some(sum_of_repeating_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
