pub fn solve() {
    let data = std::fs::read_to_string(r"data/data1.txt").expect("missing data 1");

    let (mut biggest, mut second_biggest, mut third_biggest) = (0, 0, 0);

    let mut sum = 0;
    for line in data.lines() {
        if line.is_empty() {
            match sum {
                x if x > biggest => {
                    third_biggest = second_biggest;
                    second_biggest = biggest;
                    biggest = sum;
                }
                x if x > second_biggest => {
                    third_biggest = second_biggest;
                    second_biggest = sum;
                }
                x if x > third_biggest => {
                    third_biggest = sum;
                }
                _ => {}
            }

            sum = 0;
            continue;
        }

        sum = sum + line.parse::<i32>().unwrap();
    }

    println!(
        "biggest: {}, second_biggest: {}, third_biggest: {}",
        biggest, second_biggest, third_biggest
    );
    println!("total: {}", biggest + second_biggest + third_biggest);
}
