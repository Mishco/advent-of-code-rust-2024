advent_of_code::solution!(4);

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (1, -1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
];

fn count_xmas_occurrences(grid: Vec<Vec<char>>, target: &str) -> usize {
    let rows = grid.len();
    let cols = grid.len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &DIRECTIONS {
                count += check_direction(&grid, i, j, dx, dy, target);
            }
        }
    }

    count
}

fn check_direction(
    grid: &[Vec<char>], //&Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
    target: &str,
) -> usize {
    let mut x = start_x as isize;
    let mut y = start_y as isize;
    let target_len = target.len();
    let mut chars = Vec::new();

    for _ in 0..target_len {
        // Check we are within table
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
            return 0;
        }
        chars.push(grid[x as usize][y as usize]);
        x += dx;
        y += dy;
    }

    if chars.iter().collect::<String>() == target {
        return 1;
    }

    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let chars = input.lines().map(|line| line.chars().collect()).collect();
    let result = count_xmas_occurrences(chars, "XMAS");

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result: u32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'A' && i > 0 && j > 0 && i < grid[0].len() - 1 && j < grid.len() - 1 {
                let c1 = grid[i - 1][j - 1];
                let c2 = grid[i + 1][j + 1];
                let c3 = grid[i - 1][j + 1];
                let c4 = grid[i + 1][j - 1];
                if ((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M'))
                    && ((c3 == 'M' && c4 == 'S') || (c3 == 'S' && c4 == 'M'))
                {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
