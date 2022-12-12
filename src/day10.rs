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

use std::collections::HashMap;

pub fn solve() -> Result<(), io::Error> {
    let data = std::fs::read_to_string(r"data10.txt").expect("missing data 10");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut ins = vec![];
    for line in data.lines().rev() {
        if let Some((_, part_two)) = line.split_once(' ') {
            ins.push(Instruction::Addx {
                value: part_two.parse::<i32>().unwrap(),
            })
        } else {
            ins.push(Instruction::Noop);
        }
    }

    let mut total_signal = 0;
    let mut machine = Machine::new(ins);

    while machine.execution_in_progress {
        machine.run();

        //Rendering
        let mut span_buffer = vec![];

        for line in &machine.display_buffer {
            let mut spans_vec = vec![];
            for pixel in line {
                let span = Span::raw(String::from(*pixel));
                spans_vec.push(span);
            }

            span_buffer.push(Spans::from(spans_vec));
        }

        let p = Paragraph::new(span_buffer)
            .block(
                Block::default()
                    .title("Elf CRT")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(251, 251, 199))),
            )
            .style(Style::default().fg(Color::LightGreen))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(p, size);
        })?;

        //Part 1
        match machine.cycle + 1 {
            20 | 60 | 100 | 140 | 180 | 220 => {
                total_signal += (machine.cycle + 1) * machine.x_reg;
            }
            _ => {}
        }
    }

    //println!("signal strength: {}", total_signal);
    Ok(())
}

#[derive(Default)]
struct Machine {
    x_reg: i32,
    cycle: i32,
    instructions: Vec<Instruction>,
    current_instruction: Option<Instruction>,
    remaining_cycles: i32,
    execution_in_progress: bool,
    display_buffer: Vec<Vec<char>>,
}

impl Machine {
    fn run(&mut self) {
        //Pre
        if let None = self.current_instruction {
            //Look for next instructions
            if self.instructions.len() > 0 {
                let new_instruction = self.instructions.pop();
                self.current_instruction = new_instruction;
                if let Some(ins) = new_instruction {
                    match ins {
                        Instruction::Addx { value: _ } => self.remaining_cycles = 2,
                        Instruction::Noop => self.remaining_cycles = 1,
                    }
                }
            }
            //No more instructions in memory, flag as finished
            else {
                self.execution_in_progress = false;
                return;
            }
        }

        //During
        self.update_display_buffer();

        //After
        if let Some(instruction) = self.current_instruction {
            //Execute the current instructions if there are cycles remaining
            if self.remaining_cycles == 1 {
                self.execute(instruction);
            }
            //Decrese the remaining cycles for current instructions
            else {
                self.remaining_cycles -= 1;
            }

            self.cycle += 1;
        }
    }

    fn update_display_buffer(&mut self) {
        let b_line = ((self.cycle) / 40) as usize;
        let b_pixel = ((self.cycle) % 40) as usize;

        match b_pixel as i32 - self.x_reg {
            -1 | 0 | 1 => self.display_buffer[b_line][b_pixel] = '#',
            _ => {}
        }
    }

    fn execute(&mut self, ins: Instruction) {
        match ins {
            Instruction::Addx { value } => self.addx(value),
            Instruction::Noop => {}
        }

        self.current_instruction = None;
    }

    fn addx(&mut self, value: i32) {
        self.x_reg += value;
    }

    fn new(instructions: Vec<Instruction>) -> Self {
        return Machine {
            x_reg: 1,
            cycle: 0,
            instructions,
            current_instruction: None,
            remaining_cycles: 0,
            execution_in_progress: true,
            display_buffer: vec![vec![' '; 40]; 6],
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Addx { value: i32 },
    Noop,
}
