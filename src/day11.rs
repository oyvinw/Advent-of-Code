pub fn solve() {
    let data = std::fs::read_to_string("data11.txt").expect("missing data 11");

    let mut monkeys = vec![Monkey::default(); 8];
    let mut monkey_business_index = vec![0; 8];

    let monkey = Monkey {
        items: vec![],
        operation: Box::new(add),
        recieving_monkeys: (0, 1),
        test_div: 0,
    }

    for round in 0..20 {
        for i in monkeys.len() {
            for j in monkeys[i].items {
                //do operation on worry level
                //divide worry level by 3
                //check if worry level is disivible by test_div
                //Throw to correct monkey

                //count items thrown for each monkey
                monkey_business_index[i] += 1;
            }
        }
    }
}

#[derive(Default, Copy, Clone)]
struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32, i32) -> i32>,
    operation_var: i32,
    recieving_monkeys: (usize, usize),
    test_div: i32,
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
