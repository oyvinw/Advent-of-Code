pub fn solve() {
    let data = std::fs::read_to_string(r"data2.txt").expect("missing data 2");
    let mut points = 0;

    for line in data.lines() {
        match line.chars().nth(2).unwrap() {
            'X' => {
                points = points + 1;
                match line.chars().nth(0).unwrap() {
                    'A' => points += 3,
                    'C' => points += 6,
                    _ => {}
                }
            }

            'Y' => {
                points = points + 2;
                match line.chars().nth(0).unwrap() {
                    'B' => points += 3,
                    'A' => points += 6,
                    _ => {}
                }
            }

            'Z' => {
                points = points + 3;
                match line.chars().nth(0).unwrap() {
                    'C' => points += 3,
                    'B' => points += 6,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("first_task: {}", points);

    points = 0;
    for line in data.lines() {
        match line.chars().nth(2).unwrap() {
            'X' => match line.chars().nth(0).unwrap() {
                'A' => points += 3,
                'B' => points += 1,
                'C' => points += 2,
                _ => {}
            },

            'Y' => {
                points = points + 3;
                match line.chars().nth(0).unwrap() {
                    'A' => points += 1,
                    'B' => points += 2,
                    'C' => points += 3,
                    _ => {}
                }
            }

            'Z' => {
                points = points + 6;
                match line.chars().nth(0).unwrap() {
                    'A' => points += 2,
                    'B' => points += 3,
                    'C' => points += 1,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("second_task: {}", points);
}
