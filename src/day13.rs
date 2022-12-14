pub fn solve() {
    let data = std::fs::read_to_string(r"testdata13.txt").expect("missing data 13");

    let mut pairs: Vec<(Item, Item)> = vec![];
    for line in data.lines() {
        let parent_item = Item::default();
        for s in line.split(',') {
            match s {
                "[" => {},
                "]" => {},
            }
            
        }
    }
}

#[derive(Default)]
struct Item {
    items: Option<Vec<Item>>,
    value Option<i32>,
}
