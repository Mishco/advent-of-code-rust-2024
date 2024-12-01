use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // println!("{}", input);
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in input.lines(){
        // let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 2 { // Ensure we have two numbers
            column1.push(numbers[0].clone());
            column2.push(numbers[1].clone());
        }
    }
    println!("{:?}", column1);
    println!("{:?}", column2);

    column1.sort();//sort_by(|a, b| a.cmp(b));
    column2.sort();// (|a, b| a.cmp(b));

    // let result:i32 = column1.iter()
    //     .zip(column1.iter())
    //     .map(|(&left, &right)| (left - right).abs())
    //     .sum();
    let mut result: i32 = 0;
    for i in 0..column1.len() {
        let left = column1[i];
        let right = column2[i];
        result += (left - right).abs();
    }

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in input.lines(){
        // let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 2 {
            column1.push(numbers[0].clone());
            column2.push(numbers[1].clone());
        }
    }
    // println!("{:?}", column1);
    // println!("{:?}", column2);

    column1.sort();
    column2.sort();

    let mut count_map = HashMap::new();
    for &num in &column2 {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut result: i32 = 0;
    for i in 0..column1.len() {
        let left = column1[i];

        let count = count_map.get(&left).unwrap_or(&0);
        result += left *count;
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
