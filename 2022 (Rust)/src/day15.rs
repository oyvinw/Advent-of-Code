pub fn solve () {
    let data = std::fs::read_to_string(r"data/testdata15.txt").expect("missing data 15");

    let offset = 2;
    let mut emptyCount = 0;

    //parse
    for line in data.lines() {
       let (s_data, b_data) = line.split_once(": closest beacon is at ").unwrap();
       let (sx_data, sy_data) = s_data.split_once(", ").unwrap();
       let sx = (sx_data.split_once("Sensor at x=").unwrap().1.parse::<i32>().unwrap() + offset) as usize;
       let sy = (sy_data.split_once("y=").unwrap().1.parse::<i32>().unwrap() + offset) as usize;


       let (bx_data, by_data) = b_data.split_once(", ").unwrap();
       let bx = (bx_data.split_once("x=").unwrap().1.parse::<i32>().unwrap() + offset) as usize;
       let by = (by_data.split_once("y=").unwrap().1.parse::<i32>().unwrap() + offset) as usize;


       println!("sensor: {},{}, beacon: {},{}", sx, sy, bx, by);
    }
}

fn empty_positions(sensor: Pos, beacon: Pos) -> Vec<Pos> {
    let distance = (sensor.x - beacon.x) + (sensor.y - beacon.y);
    let mut return_positions = vec![];

    return_positions.push(Pos {x: 0, y: 0});
    return_positions.push(Pos {x: 0, y: distance});
    return_positions.push(Pos {x: distance, y: 0});
    return_positions.push(Pos {x: distance, y: distance});

    return_positions
}


struct Pos {
    x: i32,
    y: i32,
}
