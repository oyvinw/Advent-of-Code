pub fn solve() {
    let data = std::fs::read_to_string(r"data9.txt").expect("missing data 9");

    let mut rope = vec![Pos { x: 0, y: 0 }; 10];
    let mut traversed = vec![Pos { x: 0, y: 0 }];

    //parse input
    let mut instructions = vec![];
    for line in data.lines() {
        let (dir, times) = line.split_once(' ').unwrap();
        for _ in 0..(times.parse::<i32>().unwrap()) {
            match dir {
                "L" => instructions.push(Pos { x: 1, y: 0 }),
                "R" => instructions.push(Pos { x: -1, y: 0 }),
                "U" => instructions.push(Pos { x: 0, y: 1 }),
                "D" => instructions.push(Pos { x: 0, y: -1 }),
                _ => {}
            }
        }
    }

    //execute instructions
    for i in 0..instructions.len() {
        rope[0].x += instructions[i].x;
        rope[0].y += instructions[i].y;
        update_rope(&mut rope);

        if !traversed.contains(&rope[rope.len() - 1]) {
            traversed.push(rope[rope.len() - 1]);
        }
    }

    println!("{}", traversed.len());
}

fn update_rope(rope: &mut Vec<Pos>) {
    for i in 1..rope.len() {
        let head = rope[i - 1];
        let tail = rope[i];

        //Straight
        if head.x == tail.x {
            if head.y > tail.y + 1 {
                rope[i].y += 1;
            }

            if head.y < tail.y - 1 {
                rope[i].y -= 1;
            }
        }

        if head.y == tail.y {
            if head.x > tail.x + 1 {
                rope[i].x += 1;
            }

            if head.x < tail.x - 1 {
                rope[i].x -= 1;
            }
        }

        //Diagonal up-right
        if head.x - tail.x == 2 && (head.y - tail.y == 1 || head.y - tail.y == 2)
            || head.x - tail.x == 1 && head.y - tail.y == 2
        {
            rope[i].x += 1;
            rope[i].y += 1;
        }

        //Diagonal down-right
        if head.x - tail.x == 2 && (head.y - tail.y == -1 || head.y - tail.y == -2)
            || head.x - tail.x == 1 && head.y - tail.y == -2
        {
            rope[i].x += 1;
            rope[i].y -= 1;
        }

        //Diagonal down-left
        if head.x - tail.x == -2 && (head.y - tail.y == -1 || head.y - tail.y == -2)
            || head.x - tail.x == -1 && head.y - tail.y == -2
        {
            rope[i].x -= 1;
            rope[i].y -= 1;
        }

        //Diagonal up-left
        if head.x - tail.x == -2 && (head.y - tail.y == 1 || head.y - tail.y == 2)
            || head.x - tail.x == -1 && head.y - tail.y == 2
        {
            rope[i].x -= 1;
            rope[i].y += 1;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
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
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, Paragraph, Widget, Wrap},
    Terminal,
};

pub fn fancy_terminal() -> Result<(), io::Error> {
    let data = std::fs::read_to_string(r"data9.txt").expect("missing data 9");

    let mut rope = vec![Pos { x: 0, y: 0 }; 10];
    let mut traversed = vec![Pos { x: 0, y: 0 }];

    //parse input
    let mut instructions = vec![];
    for line in data.lines() {
        let (dir, times) = line.split_once(' ').unwrap();
        for _ in 0..(times.parse::<i32>().unwrap()) {
            match dir {
                "L" => instructions.push(Pos { x: 1, y: 0 }),
                "R" => instructions.push(Pos { x: -1, y: 0 }),
                "U" => instructions.push(Pos { x: 0, y: 1 }),
                "D" => instructions.push(Pos { x: 0, y: -1 }),
                _ => {}
            }
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //execute instructions
    for i in 0..instructions.len() {
        rope[0].x += instructions[i].x;
        rope[0].y += instructions[i].y;
        update_rope(&mut rope);

        if !traversed.contains(&rope[rope.len() - 1]) {
            traversed.push(rope[rope.len() - 1]);
        }

        let mut points: Vec<(f64, f64)> = rope
            .iter()
            .map(|point| (f64::from(point.x), f64::from(point.y)))
            .collect();
        //let mut text = text;

        let data = vec![Dataset::default()
            .name("Rope")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Green))
            .data(&points)];

        let chart = Chart::new(data)
            .block(Block::default().title("Chart"))
            .x_axis(
                Axis::default()
                    .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([-25.0, 25.0])
            )
            .y_axis(
                Axis::default()
                    .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
                    .style(Style::default().fg(Color::White))
                    .bounds([-25.0, 25.0])
            );

        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(chart, size);
        })?;
        thread::sleep(Duration::from_millis(100));
    }

    thread::sleep(Duration::from_millis(5000));

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
