fn part1() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<(i32, bool)>> = Vec::new();

    for line in input.lines() {
        grid.push(Vec::new());
        for c in line.chars() {
            let height = c.to_digit(10).unwrap() as i32;
            grid.last_mut().unwrap().push((height, false));
        }
    }

    for row in grid.iter_mut() {
        let mut max = -1;

        for (height, visible) in row.iter_mut() {
            if *height > max {
                max = *height;
                *visible = true;
            }
        }

        let mut max = -1;

        for (height, visible) in row.iter_mut().rev() {
            if *height > max {
                max = *height;
                *visible = true;
            }
        }
    }

    for column in 0..grid.first().unwrap().len() {
        let mut max = -1;

        for row in 0..grid.len() {
            let (height, visible) = &mut grid[row][column];

            if *height > max {
                max = *height;
                *visible = true;
            }
        }

        let mut max = -1;

        for row in (0..grid.len()).rev() {
            let (height, visible) = &mut grid[row][column];

            if *height > max {
                max = *height;
                *visible = true;
            }
        }
    }

    let visible_trees = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|(_, visible)| *visible)
        .count();

    println!("{}", visible_trees);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut grid: Vec<Vec<(i32, i32)>> = Vec::new();

    for line in input.lines() {
        grid.push(Vec::new());
        for c in line.chars() {
            let height = c.to_digit(10).unwrap() as i32;
            grid.last_mut().unwrap().push((height, 0));
        }
    }

    let m = grid.len();
    let n = grid.first().unwrap().len();

    for row in 1..m - 1 {
        for column in 1..n - 1 {
            grid[row][column].1 = 1;
            let mut viewing_distance = 0;

            for i in (0..row).rev() {
                viewing_distance += 1;
                if grid[i][column].0 >= grid[row][column].0 || i == 0 {
                    grid[row][column].1 *= viewing_distance;
                    break;
                }
            }

            let mut viewing_distance = 0;

            for i in (row + 1)..m {
                viewing_distance += 1;
                if grid[i][column].0 >= grid[row][column].0 || i == m - 1 {
                    grid[row][column].1 *= viewing_distance;
                    break;
                }
            }

            let mut viewing_distance = 0;

            for j in (0..column).rev() {
                viewing_distance += 1;
                if grid[row][j].0 >= grid[row][column].0 || j == 0 {
                    grid[row][column].1 *= viewing_distance;
                    break;
                }
            }

            let mut viewing_distance = 0;

            for j in (column + 1)..n {
                viewing_distance += 1;
                if grid[row][j].0 >= grid[row][column].0 || j == n - 1 {
                    grid[row][column].1 *= viewing_distance;
                    break;
                }
            }
        }
    }

    let highest_scenic_score = grid
        .iter()
        .flat_map(|r| r.iter())
        .map(|&(_, score)| score)
        .max()
        .unwrap();

    println!("{}", highest_scenic_score);
}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
