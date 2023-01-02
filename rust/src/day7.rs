use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;

#[derive(Debug)]
struct FsNode {
    name: String,
    is_dir: bool,
    size: u64,
    sub_items: HashMap<String, FsNode>
}

pub fn run() {
    let file = File::open("resources/day7input.txt").expect("Input file not found.");
    let reader = BufReader::new(file);

    let mut current_dir = VecDeque::from([String::from("_")]);
    let mut root_node = FsNode {
        name: String::from("_"),
        is_dir: true,
        size: 0,
        sub_items: HashMap::new()
    };

    let mut current_node = &mut root_node;

    for line in reader.lines() {
        let linex = line.unwrap();
        let parts: Vec<&str> = linex.split_whitespace().collect();
        if parts[0] == "$" {
            match parts[1] {
                "cd" => {
                    let dir = parts[2];
                    if dir == "/" {
                        current_dir = VecDeque::from([String::from("_")]);
                        current_node = &mut root_node;
                    } else if dir == ".." {
                        current_dir.pop_back();
                        current_node = find_node_by_path(&mut root_node,VecDeque::from(current_dir.clone())).unwrap();
                    } else {
                        current_dir.push_back(String::from(dir));
                        current_node = current_node.sub_items.get_mut(dir).unwrap();
                    }
                }
                "ls" => {}
                _ => eprintln!("error: invalid command {:?}", parts[1])
            }
        } else if parts[0] == "dir" {
            current_node.sub_items.insert(String::from(parts[1]),FsNode {
                name: String::from(parts[1]),
                is_dir: true,
                size: 0,
                sub_items: HashMap::new()
            });
        } else {
            current_node.sub_items.insert(String::from(parts[1].to_string()),FsNode {
                name: String::from(parts[1].to_string()),
                is_dir: false,
                size: parts[0].parse::<u64>().unwrap(),
                sub_items: HashMap::new()
            });
        }
    }

    let mut sizes:Vec<u64> = Vec::new();
    let mut sum: u64 = 0;
    let mut answer2: u64 = 0;
    let fs_size: u64  = 70000000;
    let fs_min_space: u64  = 30000000;
    let used_space = calculate_dir_sizes(&root_node, &mut sizes);
    let free_space = fs_size-used_space;
    println!("used space: {:?}", used_space);
    println!("free space: {:?}", free_space);

    sizes.sort();
    for x in sizes {
        if x <= 100000 {
            sum += x;
        }
        if answer2 == 0 && free_space+x >= fs_min_space {
            answer2 = x;
        }
    }

    println!("Day 7 Answer 1: {:?}", sum);
    println!("Day 7 Answer 2: {:?}", answer2);
}

fn find_node_by_path(node: &mut FsNode, mut path: VecDeque<String>) -> Option<&mut FsNode> {
    let name = path.pop_front()?; // Return None if the path is empty
    if path.len() == 0 && name == "_" {
        return Some(node);
    }
    if name == "_" {
        return find_node_by_path(node, path);
    }
    if let Some(sub_item) = node.sub_items.get_mut(name.as_str()) {
        return if path.len() == 0 && sub_item.name == name && sub_item.is_dir {
            Some(sub_item)
        } else {
            find_node_by_path(sub_item, path)
        }
    }
    None
}

fn calculate_dir_sizes(node: &FsNode, dir_sizes: &mut Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    if node.sub_items.len() > 0 {
        for (_, sub_node) in &node.sub_items {
            if sub_node.is_dir {
                let size = calculate_dir_sizes(sub_node, dir_sizes);
                sum += size;
                dir_sizes.push(size);
            } else {
                sum += sub_node.size;
            }
        }
    }
    sum
}