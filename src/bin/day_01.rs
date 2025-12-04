use std::fs;

// move struct
struct Move {
    dir: Direction,
    dist: i32,
}

enum Direction {
    Right,
    Left,
}

fn count_crossings(before: i32, after: i32) -> i32 {
    let crossings = after.abs() / 100;
    if after > 0 {
        crossings
    } else {
        crossings + i32::from(before != 0)
    }
}

fn read_moves(file: &str) -> Vec<Move> {
    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            let (turn, dist_str) = line.split_at(1);
            let dist = dist_str.trim().parse::<i32>().unwrap();
            let dir = match turn {
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!("Invalid direction"),
            };
            Move { dir, dist }
        })
        .collect()
}

fn main() {
    let moves = read_moves("day1_real.txt");

    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut loc = 50;
    for mv in moves {
        let before: i32 = loc;
        let after: i32 = match mv.dir {
            Direction::Right => loc + mv.dist,
            Direction::Left => loc - mv.dist,
        };

        if loc % 100 == 0 {
            part_1 += 1;
        }
        part_2 += count_crossings(before, after);

        loc = after.rem_euclid(100);
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
