advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    //increasing , decs
    let mut safe_count = 0;
    let mut unsafe_count = 0;
    for line in input.lines() {
        println!("{}", line);

        let numbers: Vec<i32> = line.split_whitespace().map(|item| {
            item.parse::<i32>().unwrap()
        }).collect();

        if checking_level(&numbers) {
            safe_count += 1;
        } else {
            unsafe_count += 1;
        }
    }

    println!("{}",safe_count);
        // if let Some(value) = checking_level(numbers) {
        //     return value;
        // }

        // if numbers.iter().max().unwrap() - numbers.iter().min().unwrap() > 3 {
        //     // unsafe
        //     unsafe_count+=1;
        // } else {
        //     safe_count+=1;
        // }

        // if let (Some(max), Some(min)) = (numbers.iter().max(), numbers.iter().min()) {
        //     if max - min > 3 {
        //         unsafe_count += 1;
        //     } else {
        //         safe_count += 1;
        //     }
        // }


    Some(safe_count as u32)

}

fn checking_level(numbers: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..numbers.len() - 1 {
        // let diff = (&numbers - &numbers).abs();
        let curr = numbers[i];
        let next = numbers[i + 1];

        let diff = (next - curr).abs();

        if diff < 1 || diff > 3 {
            //unsafe
            // unsafe_count += 1;
            return false
        }

        if curr < next {
            //decreasing
            // safe_count += 1;
            is_decreasing = false;
            // return false;
        } else if curr > next {
            //increas
            // safe_count += 1;
            is_increasing= false;
            // return Some(false)
        } else {
            // nochagning
            return false;
            // return Some(false);
        }
    }
    is_increasing || is_decreasing
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;
    let mut unsafe_count = 0;
    for line in input.lines() {
        println!("{}", line);

        let numbers: Vec<i32> = line.split_whitespace().map(|item| {
            item.parse::<i32>().unwrap()
        }).collect();

        if checking_level(&numbers) {
            safe_count += 1;
        } else {
            // try to remove item and check again
            for i in 0..numbers.len() {
                let mut cleaned_level = numbers.to_vec();
                cleaned_level.remove(i);


                if checking_level(&cleaned_level) {
                    //
                    safe_count += 1;
                    break;
                }
            }
            // not safe for sure
            unsafe_count += 1;
        }
        // if checking_level(&numbers) {
        //     safe_count += 1;
        // } else {
        //     unsafe_count += 1;
        // }
    }

    println!("{}",safe_count);
    Some(safe_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
