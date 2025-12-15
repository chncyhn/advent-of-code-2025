use std::cmp;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Segment {
    u: Coord,
    v: Coord,
}

impl Coord {
    fn rect(&self, other: &Coord) -> i64 {
        (1 + (self.x - other.x).abs()) * (1 + (self.y - other.y).abs())
    }
}

fn read_segments(file: &str) -> Vec<Segment> {
    let coords: Vec<Coord> = fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let locs: Vec<&str> = l.trim().split(",").into_iter().collect();
            Coord {
                x: locs[0].parse::<i64>().unwrap(),
                y: locs[1].parse::<i64>().unwrap(),
            }
        })
        .collect();
    coords
        .windows(2)
        .map(|p| Segment { u: p[0], v: p[1] })
        .collect()
}

fn part_1(segments: &Vec<Segment>) -> i64 {
    let mut best = 0;
    let coords: Vec<Coord> = segments
        .iter()
        .flat_map(|s| [s.u, s.v])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    for i in 0..coords.len() {
        let first = coords[i];
        for j in 0..i {
            let second = coords[j];
            best = cmp::max(best, first.rect(&second));
        }
    }
    best
}

fn main() {
    let segments = read_segments("day09_real.txt");

    let part_1 = part_1(&segments);
    println!("Part 1: {}", part_1);
}
