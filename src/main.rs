#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs;

fn main() {
    //day1();
    //day2();
    //day3();
}

fn day1() {
    let data = fs::read_to_string(r"data1.txt").expect("missing data 1");

    let mut biggest = 0;
    let mut second_biggest = 0;
    let mut third_biggest = 0;

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

fn day2() {
    let data = fs::read_to_string(r"data2.txt").expect("missing data 2");
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

fn day3() {
    let data = fs::read_to_string(r"data3.txt").expect("missing data 3");

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
}

fn get_priority(c: char) -> i32 {
    match c.is_lowercase() {
        true => (c as i32) - 96,
        false => (c as i32) - 38,
    }
}
