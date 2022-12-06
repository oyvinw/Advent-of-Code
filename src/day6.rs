pub fn solve() {
    let data = std::fs::read_to_string(r"data6.txt").expect("missing data 6");
    let byte_data = data.as_bytes();

    let identifier_length = 14;

    let mut result = 0;
    'outer: for i in 0..data.len() - 4 {
        let mut region = vec![];
        for j in 0..identifier_length {
            if !region.contains(&byte_data[i + j]) {
                region.push(byte_data[i + j]);
                if region.len() == identifier_length {
                    result = i + j + 1;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", result);
}

//Visualization
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
    //Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let data = std::fs::read_to_string(r"data6.txt").expect("missing data 6");
    let byte_data = data.as_bytes();

    let identifier_length = 4;

    let mut result = 0;
    'outer: for i in 0..data.len() - 4 {
        let mut region = vec![];
        for j in 0..identifier_length {
            if !region.contains(&byte_data[i + j]) {
                region.push(byte_data[i + j]);
                if region.len() == identifier_length {
                    result = i + j + 1;
                    break 'outer;
                }

            terminal.draw(|f| {
                let size = f.size();
                f.render_widget(
                    frame_text(
                        &data,
                        i,
                        j,
                        ColorSet {
                            bracket_color: Color::LightYellow,
                            highlight_color: Color::Blue,
                        },
                    ),
                    size,
                );
            })?;
            } 
        }
    }

    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(
            frame_text(
                &data,
                result - identifier_length,
                identifier_length,
                ColorSet {
                    bracket_color: Color::LightGreen,
                    highlight_color: Color::Green,
                },
            ),
            size,
        );
    })?;
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

fn frame_text(data: &str, point: usize, length: usize, cols: ColorSet) -> Paragraph {
    let (part_one, rest) = data.split_at(point);
    let (highlight_section, part_three) = rest.split_at(length);

    let styled_text = vec![Spans::from(vec![
        Span::styled(part_one, Style::default().fg(Color::DarkGray)),
        Span::styled("[", Style::default().fg(cols.bracket_color)),
        Span::styled(highlight_section, Style::default().fg(cols.highlight_color)),
        Span::styled("]", Style::default().fg(cols.bracket_color)),
        Span::styled(part_three, Style::default().fg(Color::White)),
    ])];

    Paragraph::new(styled_text)
        .block(
            Block::default()
                .title("MESSAGE FINDER 9000")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(251, 251, 199))),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

struct ColorSet {
    bracket_color: Color,
    highlight_color: Color,
}
