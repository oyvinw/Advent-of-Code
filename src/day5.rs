pub fn solve() {
    let data = std::fs::read_to_string(r"data5.txt").expect("missing data 5");

    let (stacks_data, instruction_data) = data.split_once("\n\n").unwrap();

    //parse stack data
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in stacks_data.lines().rev().skip(1) {
        for i in 0..line.chars().skip(1).count() {
            if i % 4 == 0 && !line.chars().nth(i + 1).unwrap().is_ascii_whitespace() {
                if stacks.len() < (i / 4) + 1 {
                    stacks.push(vec![]);
                }
                stacks[i / 4].push(line.chars().nth(i + 1).unwrap());
            }
        }
    }

    //parse instructions
    let mut instructions: Vec<Vec<usize>> = vec![vec![]; 3];
    for line in instruction_data.lines().rev() {
        let mut count = 0;
        for part in line.split_whitespace() {
            if is_numeric(part) {
                instructions[count].push(part.parse::<usize>().unwrap());
                count += 1;
            }
        }
    }

    //compute instructions on data
    for _ in 0..instructions[0].len() {
        let iterations = instructions[0].pop().unwrap();
        let from = instructions[1].pop().unwrap() - 1;
        let to = instructions[2].pop().unwrap() - 1;

        //part 1
        /*
        for _ in 0..iterations {
            let cargo = stacks[from].pop().unwrap();
            stacks[to].push(cargo);
        }
        */

        //part 2
        let split_at = stacks[from].len() - iterations;
        let cargo = stacks[from].split_off(split_at);
        for piece in cargo {
            stacks[to].push(piece);
        }
    }

    for vec in stacks {
        println!("{:?}", vec[vec.len() - 1]);
    }
}

fn is_numeric(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}
