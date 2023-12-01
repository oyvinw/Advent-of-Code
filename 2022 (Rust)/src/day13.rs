pub fn solve() {
    let data = std::fs::read_to_string(r"data/testdata13.txt").expect("missing data 13");

    let mut itemArena = ItemArena::default();
    let mut pairs: Vec<(Item, Item)> = vec![];
    for line in data.lines() {
        let mut parent = itemArena.item(Item::default());
        for s in line.split(',') {
            match s {
                "[" => {
                    let mut current = itemArena.item(Item::default());
                    itemArena.arena[parent].items = Some(vec![current]);
                    parent = current;
                }
                "]" => parent -= 1,
                _ => {
                    //        itemArena.arena[parent].value s.parse::<i32>().unwrap;
                }
            }
        }
    }
}

#[derive(Default, Debug, PartialEq)]
struct Item {
    index: usize,
    items: Option<Vec<usize>>,
    value: Option<i32>,
}

impl Item {
    fn new(index: usize) -> Self {
        Self {
            index,
            items: None,
            value: None,
        }
    }
}

#[derive(Default, Debug)]
struct ItemArena {
    arena: Vec<Item>,
}

impl ItemArena {
    fn item(&mut self, item: Item) -> usize {
        for i in &self.arena {
            if *i == item {
                return i.index;
            }
        }

        let index = self.arena.len();
        self.arena.push(Item::new(index));
        index
    }
}
