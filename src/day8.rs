#![allow(unused_mut)]
#![allow(unused_variables)]

pub fn solve() {
    let data = std::fs::read_to_string(r"data8.txt").expect("missing data 8");

    let mut value_map = vec![];

    for line in data.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        value_map.push(row);
    }

    let mut visible_map = calculate_visible_outside(&value_map);

    let mut total: i32 = 0;
    for y in visible_map.iter() {
        for x in y.iter() {
            total += x;
        }
    }

    let (_, _, score) = calculate_ideal_base(&value_map);
    println!("Ideal score: {}", score);

    //println!("Best tree score: {}", biggest);
}

fn calculate_ideal_base(value_map: &Vec<Vec<i32>>) -> (usize, usize, i32) {
    //PART2
    let mut biggest = 0;
    let (mut xCoord, mut yCoord) = (0, 0);

    for y in 0..value_map.len() {
        for x in 0..value_map[y].len() {
            let height = value_map[y][x];

            let (mut left, mut right, mut up, mut down) = (0, 0, 0, 0);

            'left: for xx in (0..x).rev() {
                left += 1;
                if !(value_map[y][xx] < height) {
                    break 'left;
                }
            }

            'right: for xx in (x + 1)..(value_map[y].len()) {
                right += 1;
                if !(value_map[y][xx] < height) {
                    break 'right;
                }
            }

            'down: for yy in (y + 1)..(value_map.len()) {
                down += 1;
                if !(value_map[yy][x] < height) {
                    break 'down;
                }
            }

            'up: for yy in (0..y).rev() {
                up += 1;
                if !(value_map[yy][x] < height) {
                    break 'up;
                }
            }

            let score = up * left * right * down;
            if score > biggest {
                biggest = score;
                yCoord = y;
                xCoord = x;
            }
        }
    }

    return (yCoord, xCoord, biggest);
}

fn calculate_visible_outside(value_map: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visible_map: Vec<Vec<i32>> = vec![vec![0; value_map[0].len()]; value_map.len()];

    //PART 1
    //Left to right
    for y in 0..value_map.len() {
        let mut tallest = -1;
        for x in 0..value_map[y].len() {
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    //Right to left
    for y in 0..value_map.len() {
        let mut tallest = -1;
        for x in (0..value_map[y].len()).rev() {
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    //Bottom to top
    for x in 0..value_map[0].len() {
        let mut tallest = -1;
        for y in (0..value_map.len()).rev() {
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    //Top to bottom
    for x in 0..value_map[0].len() {
        let mut tallest = -1;
        for y in 0..value_map.len() {
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    return visible_map;
}

//------------------------------------------------------
//Visualization
//------------------------------------------------------
//
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

pub fn fancy_terminal() -> Result<(), io::Error> {
    let data = std::fs::read_to_string(r"data8.txt").expect("missing data 8");

    let mut value_map = vec![];

    for line in data.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        value_map.push(row);
    }

    let visible_map = calculate_visible_outside(&value_map);
    let (baseY, baseX, _) = calculate_ideal_base(&value_map);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut text = vec![];

    let mut total: i32 = 0;
    for y in 0..visible_map.len() {
        let mut spans_vec = vec![];
        for x in 0..visible_map[y].len() {
            let value = value_map[y][x];
            let mut span = Span::styled(format!("{}", value), Style::default().fg(Color::DarkGray));
            if visible_map[y][x] == 1 {
                span.style = Style::default().fg(Color::Green);
            }
            if x == baseX && y == baseY {
                span.style = Style::default().fg(Color::Blue);
            }

            spans_vec.push(span);
        }

        text.push(Spans::from(spans_vec));
    }

    let p = Paragraph::new(text)
        .block(
            Block::default()
                .title("TREE SCANNER")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(251, 251, 199))),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(p, size);
    })?;

    thread::sleep(Duration::from_millis(50000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
