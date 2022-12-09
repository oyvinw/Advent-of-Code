use std::rc;

pub fn solve() {
    let mut file_tree = FileArena::default();
    let data = std::fs::read_to_string(r"data7.txt").expect("missing data 7");

    //parse tree
    let mut current_node = file_tree.node(Folder {
        name: format!("/"),
        size: 0,
    });

    for line in data.lines().skip(1) {
        let (first, content) = line.split_once(' ').unwrap();
        match first {
            "$" => {
                if let Some((second, content)) = content.split_once(' ') {
                    match second {
                        "cd" => match content {
                            ".." => {
                                if let Some(node) = file_tree.arena[current_node].parent {
                                    current_node = node;
                                }
                            }
                            _ => {
                                current_node = file_tree.node(Folder {
                                    name: format!(
                                        "{}/{}",
                                        &file_tree.arena[current_node].folder.name,
                                        content.to_string()
                                    ),
                                    size: 0,
                                })
                            }
                        },
                        _ => {}
                    }
                }
            }
            "dir" => {
                let parent_name = &file_tree.arena[current_node].folder.name;
                let dir = file_tree.node(Folder {
                    name: format!("{}/{}", parent_name, content.to_string()),
                    size: 0,
                });

                file_tree.add_child(current_node, dir);
            }
            _ => {
                if first.chars().nth(0).unwrap().is_numeric() {
                    file_tree.arena[current_node].folder.size += first.parse::<u64>().unwrap();
                }
            }
        }
    }

    let total_system_size = 70000000;
    let space_needed = 30000000;

    let mut total_under_100k = 0;
    let mut smallest = 1000000000;

    let total_used = file_tree.do_calc(0, &mut total_under_100k, &mut 0, 0);
    println!("sum of all sub 100k: {}", total_under_100k);
    println!("total used: {} of {}", total_used, total_system_size);

    let required_space = (total_used + space_needed) - total_system_size;

    file_tree.do_calc(0, &mut total_under_100k, &mut smallest, required_space);

    println!(
        "{} space needed, smallest folder: {}",
        required_space, smallest
    );
}

#[derive(Debug, Default)]
struct Node {
    index: usize,
    folder: Folder,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(index: usize, folder: Folder) -> Self {
        Self {
            index,
            folder,
            parent: None,
            children: vec![],
        }
    }
}

#[derive(PartialEq, Debug, Default)]
struct Folder {
    name: String,
    size: u64,
}

#[derive(Default, Debug)]
struct FileArena {
    arena: Vec<Node>,
}

impl FileArena {
    fn node(&mut self, folder: Folder) -> usize {
        for node in &self.arena {
            if node.folder == folder {
                return node.index;
            }
        }

        let index = self.arena.len();
        self.arena.push(Node::new(index, folder));
        index
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        self.arena[parent].children.push(child);
        self.arena[child].parent = Some(parent);
    }

    fn do_calc(&self, node: usize, total: &mut u64, smallest: &mut u64, space_needed: u64) -> u64 {
        let mut child_sum = 0;

        for child_node in &self.arena[node].children {
            child_sum += self.do_calc(*child_node, total, smallest, space_needed);
        }

        let folder_sum = child_sum + &self.arena[node].folder.size;

        if folder_sum < 100000 {
            *total += folder_sum;
        }

        if folder_sum > space_needed && folder_sum < *smallest {
            *smallest = folder_sum;
        }

        folder_sum
    }
}
