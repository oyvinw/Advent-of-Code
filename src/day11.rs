pub fn solve() {
    let data = std::fs::read_to_string("data11.txt").expect("missing data 11");

    //parsing
    let mut monkeys: Vec<Monkey> = vec![];

    for line in data.lines() {
        if let Some((mut first, mut second)) = line.split_once(':') {
            first = first.trim();
            second = second.trim();
            match first_word(first) {
                "Monkey" => monkeys.push(Monkey {
                    items: vec![],
                    operation: Operation::Add { value: 0 },
                    recieving_monkeys: vec![],
                    test_div: 0,
                }),
                "Starting" => {
                    let str_items = second.split(", ");
                    for str_item in str_items {
                        let idx = monkeys.len() - 1;
                        monkeys[idx].items.push(str_item.parse::<u64>().unwrap());
                    }
                }
                "Operation" => {
                    let sign = second.split_whitespace().nth(3).unwrap();
                    let value = second.split_whitespace().nth(4).unwrap();

                    let idx = monkeys.len() - 1;
                    match value {
                        //Cheeky assumption that the only self operations are multiplications
                        "old" => monkeys[idx].operation = Operation::MultiplySelf,
                        _ => match sign {
                            "+" => monkeys[idx].operation = Operation::Add { value: value.parse::<u64>().unwrap() },
                            "*" => monkeys[idx].operation = Operation::Multiply { value: value.parse::<u64>().unwrap() },
                            _ => {}
                        },
                    }
                }
                "Test" => {
                    let idx = monkeys.len() - 1;
                    let (_, str_value) = second.split_once("by ").unwrap();
                    monkeys[idx].test_div = str_value.parse::<u64>().unwrap();
                }
                "If" => {
                    let idx = monkeys.len() - 1;
                    let (_, b) = first.split_once(' ').unwrap();
                    match b {
                        "true" | "false" => {monkeys[idx].recieving_monkeys.push(second.split_whitespace().nth(3).unwrap().parse::<usize>().unwrap())},
                        _ => {},
                    }
                }
                _ => {}
            }
        }
    }

    let mut monkey_business_index = vec![0; monkeys.len()];

    //computation
    let mut modulo: u64 = 1; 
    for monkey in &monkeys {
        modulo *= monkey.test_div;
    }

    //push items to monkeys, take from idx 0
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                //remove first item
                let mut item = monkeys[i].items.remove(0);
                monkey_business_index[i] += 1;
                //do operation on worry level
                item = evaluate_operation(monkeys[i].operation, item);
                //divide worry level by 3
                //item = item / 3;
                item = item % modulo;
                //check if worry level is disivible by test_div
                if item % monkeys[i].test_div == 0 {
                    //Throw to correct monkey
                    let recieving = monkeys[i].recieving_monkeys[0];
                    monkeys[recieving].items.push(item);
                } else {
                    //Throw to correct monkey
                    let recieving = monkeys[i].recieving_monkeys[1];
                    monkeys[recieving].items.push(item);
                }
            }
        }
    }

    monkey_business_index.sort_unstable();
    println!("{:?}", monkey_business_index.pop().unwrap() as u128 * monkey_business_index.pop().unwrap() as u128); 
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    recieving_monkeys: Vec<usize>,
    test_div: u64,
}

fn evaluate_operation(op: Operation, item: u64) -> u64 {
    match op {
        Operation::Add { value } => item + value,
        Operation::Multiply { value } => item * value,
        Operation::MultiplySelf => item * item,
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add { value: u64 },
    Multiply { value: u64 },
    MultiplySelf,
}

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }

    &string[..]
}
