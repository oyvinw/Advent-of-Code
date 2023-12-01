use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{cmp, io, thread, time::Duration};
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

    let data = std::fs::read_to_string(r"data/data14.txt").expect("missing data 14");

    let mut cave: Vec<Vec<Entity>> = vec![vec![Entity::Air; 10000]; 200];
    let (mut lowest_x, mut highest_x, mut lowest_y, mut highest_y) = (usize::MAX, 0, usize::MAX, 0);

    //Parsing
    for line in data.lines() {
        let mut prev_rock: Option<(usize, usize)> = None;
        for split in line.split(" -> ") {
            let (str_x, str_y) = split.split_once(",").unwrap();

            let this_x = str_x.parse::<usize>().unwrap();
            let this_y = str_y.parse::<usize>().unwrap();

            if this_x > highest_x {
                highest_x = this_x + 1;
            }

            if this_x < lowest_x {
                lowest_x = this_x;
            }

            if this_y > highest_y {
                highest_y = this_y + 2;
            }

            if this_y < lowest_y {
                lowest_y = this_y - 2;
            }

            if let Some((prev_x, prev_y)) = prev_rock {
                let from_x = cmp::min(this_x, prev_x);
                let to_x = cmp::max(this_x, prev_x);

                let from_y = cmp::min(this_y, prev_y);
                let to_y = cmp::max(this_y, prev_y);

                for y in from_y..to_y + 1 {
                    for x in from_x..to_x + 1 {
                        cave[y][x] = Entity::Rock;
                    }
                }
            }

            prev_rock = Some((this_x, this_y));
        }
    }

    //Simulate sand
    let mut sand_count = 0;
    'sand: for i in 0..1000000 {
        let mut current_x = 500;
                if cave[0][500] == Entity::Sand {
                    break 'sand;
                }
        'y: for y in 0..(highest_y + 1) {
            match cave[y][current_x] {
                Entity::Sand | Entity::Rock => {
                    if cave[y][current_x - 1] == Entity::Air {
                        current_x -= 1;
                        continue 'y;
                    }
                    if cave[y][current_x + 1] == Entity::Air {
                        current_x += 1;
                        continue 'y;
                    }

                    cave[y - 1][current_x] = Entity::Sand;
                    sand_count += 1;
                    continue 'sand;
                }
                Entity::Air => { 
                    if y == highest_y {
                        cave[y - 1][current_x] = Entity::Sand;
                        sand_count += 1;
                        continue 'sand;
                    }
                }
            }
        }
    }

            //Drawing
            let mut span_buffer = vec![];
            for y in 0..highest_y {
                let mut spans_vec = vec![];
                for x in lowest_x - 10..highest_x + 10 {
                    let mut bg_color = Color::Black;
                    if x == 500 {
                        bg_color = Color::Yellow;
                    }

                    match cave[y][x] {
                        Entity::Air => spans_vec.push(Span::styled(
                            ".",
                            Style::default().fg(Color::LightBlue).bg(bg_color),
                        )),
                        Entity::Sand => spans_vec.push(Span::styled(
                            "o",
                            Style::default().fg(Color::LightYellow).bg(bg_color),
                        )),
                        Entity::Rock => spans_vec.push(Span::styled(
                            "#",
                            Style::default().fg(Color::Gray).bg(bg_color),
                        )),
                    }
                }
                span_buffer.push(Spans::from(spans_vec));
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
            thread::sleep(Duration::from_millis(5000));

    println!("{}", sand_count);
    //println!("max: {}, min: {}", highest_x, lowest_x);

    Ok(())
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Entity {
    Sand,
    Air,
    Rock,
}
