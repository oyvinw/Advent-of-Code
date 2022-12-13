use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn solve() {
    //parsing
    let data = std::fs::read_to_string("data12.txt").expect("missing data 12");

    let mut graph = Graph::default();
    let mut start = Pos::default();
    let mut end = Pos::default();

    let mut y = 0;
    for line in data.lines() {
        let mut x = 0;
        let mut line_vec = vec![];
        for b in line.as_bytes() {
            match *b {
                83 => {
                    line_vec.push(1 as i8);
                    start = Pos { x, y };
                }
                69 => {
                    line_vec.push(26 as i8);
                    end = Pos { x, y };
                }
                _ => {
                    line_vec.push((*b - 96) as i8);
                }
            }
            x += 1;
        }
        graph.g.push(line_vec);
        y += 1;
    }

    //calculation
    let mut frontier = PriorityQueue::new();
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut cost_so_far: HashMap<Pos, i32> = HashMap::new();

    frontier.push(start, Reverse(0));
    came_from.insert(start, start);
    cost_so_far.insert(start, 0);

    //println!("{:?}", graph);
    //println!("start: {:?} end: {:?}", start, end);

    while frontier.len() > 0 {
        if let Some((current_pos, _)) = frontier.pop() {
            if current_pos == end {
                break;
            }

            println!("current_pos: {:?}", current_pos);
            for n in graph.get_neighbours(current_pos) {
                println!("neighbour: {:?}", n);
                let new_cost = cost_so_far.get(&current_pos).unwrap() + 1;
                if let Some(oldCost) = cost_so_far.get(&n) {
                    if new_cost >= *oldCost {
                        continue;
                    }
                }

                cost_so_far
                    .entry(n)
                    .and_modify(|f| *f = new_cost)
                    .or_insert(new_cost);

                //priority??
                frontier.push(n, Reverse(new_cost));
                came_from
                    .entry(n)
                    .and_modify(|f| *f = current_pos)
                    .or_insert(current_pos);
            }
        }
    }

    let mut path = vec![];
    let mut current = end;
    while current != start {
        path.push(current);
        if let Some(next) = came_from.get(&current) {
            current = *next;
        } else {
            break;
        }
    }

    println!("{:?}", path.len());
}

#[derive(Default)]
struct Graph {
    g: Vec<Vec<i8>>,
}

impl Graph {
    fn get_neighbours(&self, node: Pos) -> Vec<Pos> {
        vec![
            Pos {
                x: node.x + 1,
                y: node.y,
            },
            Pos {
                x: node.x - 1,
                y: node.y,
            },
            Pos {
                x: node.x,
                y: node.y + 1,
            },
            Pos {
                x: node.x,
                y: node.y - 1,
            },
        ]
        .iter()
        .map(|v| *v)
        .filter(|pos| {
            pos.x < self.g[0].len() as i32
                && pos.x >= 0
                && pos.y < self.g.len() as i32
                && pos.y >= 0
                && (self.g[pos.y as usize][pos.x as usize] - self.g[node.y as usize][node.x as usize]).abs() <= 1
        })
        .collect()
    }
}

#[derive(Default, Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}
