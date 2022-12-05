pub fn solve() {
    let data = std::fs::read_to_string(r"data3.txt").expect("missing data 3");

    //part 1
    let mut total: i32 = 0;
    for line in data.lines() {
        let (comp_one, comp_two) = line.split_at(line.len() / 2);
        for c in comp_one.chars() {
            if comp_two.contains(c) {
                total += get_priority(c);
                break;
            }
        }
    }

    println!("{}", total);

    //part 2
    total = 0;
    for i in 0..data.lines().count() - 2 {
        if i % 3 == 0 {
            for c in data.lines().nth(i).unwrap().chars() {
                if data.lines().nth(i + 1).unwrap().contains(c)
                    && data.lines().nth(i + 2).unwrap().contains(c)
                {
                    total += get_priority(c);
                    break;
                }
            }
        }
    }

    println!("{}", total);

    fn get_priority(c: char) -> i32 {
        match c.is_lowercase() {
            true => (c as i32) - 96,
            false => (c as i32) - 38,
        }
    }
}
