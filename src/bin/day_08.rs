use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;

struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn dist(&self, other: &Coord) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn read_coords(file: &str) -> Vec<Coord> {
    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let locs: Vec<&str> = l.trim().split(",").into_iter().collect();
            Coord {
                x: locs[0].parse::<i64>().unwrap(),
                y: locs[1].parse::<i64>().unwrap(),
                z: locs[2].parse::<i64>().unwrap(),
            }
        })
        .collect()
}

fn main() {
    let coords = read_coords("day8_real.txt");
    let mut member: Vec<usize> = (0..coords.len()).collect();

    let mut dists: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in 0..i {
            let dist = coords[i].dist(&coords[j]);
            dists.push((-dist, i, j));
        }
    }

    for iter_num in 0..1_000_000 {
        let (_, a, b) = dists.pop().unwrap();

        // update membership
        let replace = member[b];
        for i in 0..member.len() {
            if member[i] == replace {
                member[i] = member[a];
            }
        }

        // part 1 condition
        if iter_num == 1000 {
            let mut cnts: HashMap<usize, u64> = HashMap::new();
            for m in &member {
                cnts.insert(*m, cnts.get(m).unwrap_or(&0) + 1);
            }
            let mut vals: Vec<u64> = cnts.values().copied().collect();
            vals.sort_unstable_by(|a, b| b.cmp(a)); // descending
            let part_1: u64 = vals.into_iter().take(3).product();
            println!("Part 1: {}", part_1);
        }

        // part 2 condition
        if member.windows(2).all(|w| w[0] == w[1]) {
            let part_2 = coords[a].x * coords[b].x;
            println!("Part 2: {}", part_2);
            break;
        }
    }
}
