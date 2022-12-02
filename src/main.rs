#![allow(non_snake_case)]
#![allow(dead_code)]

use std::fs;

fn main() {
    day2();
}

fn day1() {
    let data = fs::read_to_string(r"data1.txt").expect("missing data 1");

    let mut biggest = 0;
    let mut second_biggest = 0;
    let mut third_biggest = 0;

    let mut sum = 0;
    for line in data.lines()
    {
        if line.is_empty(){
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

    println!("biggest: {}, second_biggest: {}, third_biggest: {}", biggest, second_biggest, third_biggest);
    println!("total: {}", biggest + second_biggest + third_biggest);
}


fn day2() {
    let data = fs::read_to_string(r"data2.txt").expect("missing data 2");
    let mut points = 0;

    for line in data.lines()
    {
        match line.chars().nth(2).unwrap(){

            'X' => { points = points + 1;
                match line.chars().nth(0).unwrap() {
                    'A' => { points = points + 3; }
                    'C' => { points = points + 6; }
                    _ => {}
                }
            }

            'Y' => { points = points + 2;
                match line.chars().nth(0).unwrap() {
                    'B' => { points = points + 3; }
                    'A' => { points = points + 6; }
                    _ => {}
                }
            }

            'Z' => { points = points + 3;
                match line.chars().nth(0).unwrap() {
                    'C' => { points = points + 3; }
                    'B' => { points = points + 6; }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("first_task: {}", points);

    points = 0;
    for line in data.lines()
    {
        match line.chars().nth(2).unwrap(){

            'X' => { match line.chars().nth(0).unwrap() {
                    'A' => { points = points + 3 }
                    'B' => { points = points + 1; }
                    'C' => { points = points + 2; }
                    _ => {}
                }
            }

            'Y' => { points = points + 3;
                match line.chars().nth(0).unwrap() {
                    'A' => { points = points + 1; }
                    'B' => { points = points + 2; }
                    'C' => { points = points + 3; }
                    _ => {}
                }
            }

            'Z' => { points = points + 6;
                match line.chars().nth(0).unwrap() {
                    'A' => { points = points + 2; }
                    'B' => { points = points + 3; }
                    'C' => { points = points + 1; }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("second_task: {}", points);

}
