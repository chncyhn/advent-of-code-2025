use std::collections::HashMap;
use std::fs;

fn read_grid(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect()
}

fn bfs(grid: &Vec<Vec<char>>) -> (u64, u64) {
    let mut queue = HashMap::new();
    let mut frontier = HashMap::new();
    queue.insert(grid[0].iter().position(|x| *x == 'S').unwrap(), 1);

    let mut cnt = 0;
    for i in 1..grid.len() {
        frontier.clear();
        for (&j, cur_cnt) in &queue {
            match grid[i][j] {
                '.' => {
                    frontier.insert(j, *frontier.get(&j).unwrap_or(&0) + cur_cnt);
                }
                '^' => {
                    frontier.insert(j - 1, *frontier.get(&(j - 1)).unwrap_or(&0) + cur_cnt);
                    frontier.insert(j + 1, *frontier.get(&(j + 1)).unwrap_or(&0) + cur_cnt);
                    cnt += 1;
                }
                _ => {}
            }
        }
        queue.clear();
        queue.extend(&frontier);
    }

    (cnt, queue.values().sum())
}

fn main() {
    let grid = read_grid("day7_real.txt");
    let (part_1, part_2) = bfs(&grid);
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

