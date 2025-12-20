use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Node = String;
type Graph = HashMap<Node, HashSet<Node>>;
type Path = Vec<Node>;

fn read_graph(file: &str) -> Graph {
    let mut graph: Graph = Graph::new();
    fs::read_to_string(file)
        .expect("Failed to read file")
        .lines()
        .for_each(|l| {
            let words: Vec<&str> = l.trim().split_whitespace().collect();
            let u = words[0].replace(":", "");
            for v in words[1..].iter() {
                graph.entry(u.clone()).or_default().insert(v.to_string());
            }
        });
    graph
}

fn part_1(graph: &Graph) -> u64 {
    let mut cnt = 0;
    let mut queue: Vec<Path> = vec![vec!["you".to_string()]];
    while queue.len() > 0 {
        let path = queue.pop().unwrap();
        let cur: Node = path[path.len() - 1].clone();
        if cur == "out".to_string() {
            cnt += 1;
            continue;
        }
        for nxt in graph.get(&cur).unwrap() {
            let mut new_path = path.clone();
            new_path.push(nxt.clone());
            queue.push(new_path);
        }
    }
    cnt
}

fn part_2(graph: &Graph) -> u64 {
    #[derive(Eq, Hash, PartialEq, Clone)]
    struct State {
        s: String,
        fft: bool,
        dac: bool,
    }

    fn f(s: &State, cache: &mut HashMap<State, u64>, reversed_graph: &Graph) -> u64 {
        if cache.contains_key(s) {
            return cache[s];
        }
        if s.s == "svr" {
            if !s.fft && !s.dac {
                return 1;
            } else {
                return 0;
            };
        }
        if s.s == "fft" && !s.fft {
            return 0;
        }
        if s.s == "dac" && !s.dac {
            return 0;
        }

        let fft = s.fft && (s.s != "fft");
        let dac = s.dac && (s.s != "dac");
        let ret: u64 = reversed_graph[&s.s]
            .iter()
            .map(|prev| {
                return f(
                    &State {
                        s: prev.clone(),
                        fft,
                        dac,
                    },
                    cache,
                    reversed_graph,
                );
            })
            .sum();
        cache.insert(s.clone(), ret);
        ret
    }

    let mut reversed_graph: Graph = Graph::new();
    for (from, tos) in graph.iter() {
        for to in tos {
            reversed_graph
                .entry(to.clone())
                .or_default()
                .insert(from.clone());
        }
    }
    let mut cache: HashMap<State, u64> = HashMap::new();
    f(
        &State {
            s: "out".to_string(),
            fft: true,
            dac: true,
        },
        &mut cache,
        &reversed_graph,
    )
}

fn main() {
    let graph = read_graph("day11_real.txt");

    println!("Part 1: {}", part_1(&graph));
    println!("Part 2: {}", part_2(&graph));
}
