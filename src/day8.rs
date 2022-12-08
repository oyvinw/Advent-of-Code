#![allow(unused_mut)]
#![allow(unused_variables)]

pub fn solve(){
    let data = std::fs::read_to_string(r"data8.txt").expect("missing data 8");

    let mut value_map = vec![];
    let mut visible_map = vec![];

    for line in data.lines(){
        let mut row = vec![];
        let mut v_row = vec![];
        for c in line.chars(){
            row.push(c.to_digit(10).unwrap() as i32); 
            v_row.push(0);
        }
        value_map.push(row);
        visible_map.push(v_row);
    }

    //PART 1
    //Left to right
    for y in 0..value_map.len(){
        let mut tallest = -1;
        for x in 0..value_map[y].len(){
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    //Right to left
    for y in 0..value_map.len(){
        let mut tallest = -1;
        for x in (0..value_map[y].len()).rev(){
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    //Bottom to top
    for x in 0..value_map[0].len(){
        let mut tallest = -1;
        for y in (0..value_map.len()).rev(){
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    //Top to bottom
    for x in 0..value_map[0].len(){
        let mut tallest = -1;
        for y in 0..value_map.len(){
            let value = value_map[y][x];
            if value > tallest {
                visible_map[y][x] = 1;
                tallest = value;
            }
        }
    }

    let mut total: i32 = 0;
    for y in visible_map.iter() {
        for x in y.iter() {
            total += x;
        }
    }

    //println!("{:?}", total);

    //PART2
    let mut biggest = 0;
    for y in 0..value_map.len(){
        for x in 0..value_map[y].len(){
            let height = value_map[y][x];

            let (mut left, mut right, mut up, mut down) = (0, 0, 0, 0);

            'left: for xx in (0..x).rev() {
                left += 1;
                if !(value_map[y][xx] < height) {
                    break 'left;
                }
            }

            'right: for xx in (x + 1)..(value_map[y].len()) {
                right += 1;
                if !(value_map[y][xx] < height) {
                    break 'right;
                }
            }

            'down: for yy in (y + 1)..(value_map.len()) {
                down += 1;
                if !(value_map[yy][x] < height) {
                    break 'down;
                }
            }

            'up: for yy in (0..y).rev() {
                up += 1;
                if !(value_map[yy][x] < height) {
                    break 'up;
                }
            }

            let score = up * left * right * down;
            if score > biggest {
                biggest = score;
            }
        }
    }

    println!("Best tree score: {}", biggest);

}
