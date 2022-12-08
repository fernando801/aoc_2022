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

fn part2() {}

fn main() {
    println!("Part 1:");
    part1();

    println!("");

    println!("Part 2:");
    part2();
}
