pub fn solve() {

    let data = std::fs::read_to_string(r"data6.txt").expect("missing data 6");
    let byte_data = data.as_bytes();

    let identifier_length = 14;

    let mut result = 0;
    'outer: for i in 0..data.len()-4{
        let mut region = vec![];
        for j in 0..identifier_length{
            if !region.contains(&byte_data[i + j]){
                region.push(byte_data[i + j]);
                if region.len() == identifier_length {
                    result = i + j + 1;
                    break 'outer;
                }
            }
        }
    }

    println!("{}", result);
}
