pub fn solve() {
    let data = std::fs::read_to_string(r"data/data4.txt").expect("missing data 4");

    let mut result = 0;
    for line in data.lines() {
        let (part_one, part_two) = line.split_once(",").unwrap();
        let (one_lower, one_higher) = part_one.split_once("-").unwrap();
        let (two_lower, two_higher) = part_two.split_once("-").unwrap();

        let one_lower_i32 = one_lower.parse::<i32>().unwrap();
        let one_higher_i32 = one_higher.parse::<i32>().unwrap();
        let two_lower_i32 = two_lower.parse::<i32>().unwrap();
        let two_higher_i32 = two_higher.parse::<i32>().unwrap();

        if ((one_lower_i32 >= two_lower_i32) && (one_higher_i32 <= two_higher_i32))
            || ((one_lower_i32 <= two_lower_i32) && (one_higher_i32 >= two_higher_i32))
        {
            result += 1; //part 1
        } else if !(one_lower_i32 > two_higher_i32 || two_lower_i32 > one_higher_i32) {
            result += 1; //part 2
        }
    }

    println!("{}, of {}", result, data.lines().count());
}
