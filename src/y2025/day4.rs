pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn processes_paper(input: &mut Vec<Vec<char>>) -> u32 {
    let mut new_grid = input.clone();
    let height = input.len();
    let width = input[0].len();
    let mut accessable = 0;
    let h = height as isize;
    let w = width as isize;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for y in 0..h {
        let uy = y as usize;
        for x in 0..w {
            let ux = x as usize;
            if input[uy][ux] != '@' {
                continue;
            }
            let n = directions
                .iter()
                .filter(|&&(dy, dx)| {
                    let ny = y + dy;
                    let nx = x + dx;
                    ny >= 0
                        && nx >= 0
                        && (ny as usize) < height
                        && (nx as usize) < width
                        && input[ny as usize][nx as usize] == '@'
                })
                .count();
            if n < 4 {
                accessable += 1;
                new_grid[uy][ux] = '.';
            }
        }
    }
    *input = new_grid.clone();
    accessable
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    processes_paper(&mut grid).to_string()
}

fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut removed = 0;
    let mut accessable = 1u32;
    while accessable != 0 {
        accessable = processes_paper(&mut grid);
        removed += accessable;
    }

    removed.to_string()
}
