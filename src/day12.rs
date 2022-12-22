use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
    Terminal,
};

pub fn solve() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //parsing
    let data = std::fs::read_to_string("data/data12.txt").expect("missing data 12");

    let mut graph = Graph::default();
    let mut start = Pos::default();
    let mut end = Pos::default();

    let mut start_vec = vec![];
    let mut shortest = usize::MAX;

    let mut y = 0;
    for line in data.lines() {
        let mut x = 0;
        let mut line_vec = vec![];
        for b in line.chars() {
            match b {
                'S' => {
                    line_vec.push('a');
                    start_vec.push(Pos { x, y });
                    start = Pos { x, y };
                }
                'E' => {
                    line_vec.push('z');
                    end = Pos { x, y };
                }
                'a' => {
                    start_vec.push(Pos { x, y });
                    line_vec.push(b);
                }
                _ => {
                    line_vec.push(b);
                }
            }
            x += 1;
        }
        graph.g.push(line_vec);
        y += 1;
    }

    //part 2
    //for start in start_vec {
    //calculation
    let mut frontier = PriorityQueue::new();
    let mut came_from: HashMap<Pos, Pos> = HashMap::new();
    let mut cost_so_far: HashMap<Pos, i32> = HashMap::new();

    frontier.push(start, Reverse(0));
    came_from.insert(start, start);
    cost_so_far.insert(start, 0);

    while frontier.len() > 0 {
        if let Some((current_pos, _)) = frontier.pop() {
            if current_pos == end {
                break;
            }

            for n in graph.get_neighbours(current_pos) {
                let mut new_cost = cost_so_far.get(&current_pos).unwrap() + 1;
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
                new_cost += manhattan_distance(n, end);
                frontier.push(n, Reverse(new_cost));
                came_from
                    .entry(n)
                    .and_modify(|f| *f = current_pos)
                    .or_insert(current_pos);
            }
        }

        //visualization
        let mut span_buffer = vec![];

        let mut y = 0;
        for line in &graph.g {
            let mut spans_vec = vec![];
            let mut x = 0;
            for pixel in line {
                let mut style = Style::default();
                if let Some(_) = frontier.get(&Pos { y, x }) {
                    style = Style::default().fg(Color::Red);
                } else if came_from.contains_key(&Pos { y, x }) {
                    style = Style::default().fg(Color::Blue);
                }
                let span = Span::styled(format!("{}", *pixel as char), style);
                spans_vec.push(span);
                x += 1;
            }

            span_buffer.push(Spans::from(spans_vec));
            y += 1;
        }

        let p = Paragraph::new(span_buffer)
            .block(
                Block::default()
                    .title("PATHFINDER")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(251, 251, 199))),
            )
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Center);
        //.wrap(Wrap { trim: true });

        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(p, size);
        })?;
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

        let mut span_buffer = vec![];

        let mut y = 0;
        for line in &graph.g {
            let mut spans_vec = vec![];
            let mut x = 0;
            for pixel in line {
                let mut style = Style::default();
                if let Some(_) = frontier.get(&Pos { y, x }) {
                    style = Style::default().fg(Color::Red);
                } else if came_from.contains_key(&Pos { y, x }) {
                    style = Style::default().fg(Color::Blue);
                }

                if path.contains(&Pos { y, x }) {
                    style = Style::default().fg(Color::Yellow);
                }

                let span = Span::styled(format!("{}", *pixel as char), style);
                spans_vec.push(span);
                x += 1;
            }

            span_buffer.push(Spans::from(spans_vec));
            y += 1;
        }

        let p = Paragraph::new(span_buffer)
            .block(
                Block::default()
                    .title("PATHFINDER")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(251, 251, 199))),
            )
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Center);
        //.wrap(Wrap { trim: true });

        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(p, size);
        })?;
    }

    /*
     * part 2
    if path.len() > 1 && path.len() < shortest {
        shortest = path.len()
    }
    */

    //println!("path calculated, length: {}", path.len());
    //}

    //println!("{:?}", shortest);
    Ok(())
}

#[derive(Default)]
struct Graph {
    g: Vec<Vec<char>>,
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
                && self.g[node.y as usize][node.x as usize] as i8
                    - self.g[pos.y as usize][pos.x as usize] as i8
                    >= -1
        })
        .collect()
    }
}

fn manhattan_distance(a: Pos, b: Pos) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

#[derive(Default, Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}
