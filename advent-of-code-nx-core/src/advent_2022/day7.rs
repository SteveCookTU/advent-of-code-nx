use alloc::rc::{Rc, Weak};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::cell::RefCell;
use core::str::FromStr;

struct DirectoryTree<'a> {
    root: Rc<RefCell<DirectoryNode<'a>>>,
}

struct DirectoryNode<'a> {
    parent: Option<Weak<RefCell<DirectoryNode<'a>>>>,
    name: &'a str,
    files: Vec<File>,
    directories: Vec<Rc<RefCell<DirectoryNode<'a>>>>,
}

struct File {
    size: u64,
}

impl<'a> DirectoryTree<'a> {
    fn find_sizes(
        &self,
        node: Option<Rc<RefCell<DirectoryNode<'a>>>>,
        result: &mut Vec<u64>,
    ) -> u64 {
        let node = node.unwrap_or_else(|| self.root.clone());
        let mut dir_total = 0;
        for directory in node.borrow().directories.iter() {
            dir_total += self.find_sizes(Some(directory.clone()), result);
        }
        let file_sum = node.borrow().files.iter().map(|f| f.size).sum::<u64>();
        result.push(file_sum + dir_total);
        file_sum + dir_total
    }
}

fn parse_to_tree<'a>(input: &[&'a str]) -> DirectoryTree<'a> {
    let tree = DirectoryTree {
        root: Rc::new(RefCell::new(DirectoryNode {
            parent: None,
            name: "/",
            files: vec![],
            directories: vec![],
        })),
    };

    let mut current_node = tree.root.clone();
    for line in input.iter().skip(2) {
        if line.starts_with("$ cd") {
            let name = &line[5..];
            if name == ".." {
                let new_node = current_node.borrow().parent.as_ref().unwrap().clone();
                current_node = new_node.upgrade().unwrap();
            } else {
                let new_node = current_node
                    .borrow()
                    .directories
                    .iter()
                    .find(|n| n.borrow().name == name)
                    .unwrap()
                    .clone();
                current_node = new_node;
            }
        } else if line.starts_with("dir") {
            let dir_name = &line[4..];
            current_node
                .borrow_mut()
                .directories
                .push(Rc::new(RefCell::new(DirectoryNode {
                    parent: Some(Rc::downgrade(&current_node)),
                    name: dir_name,
                    files: vec![],
                    directories: vec![],
                })));
        } else if !line.starts_with("$ ls") {
            let (size, _) = line.split_once(' ').unwrap();
            current_node.borrow_mut().files.push(File {
                size: u64::from_str(size).unwrap(),
            });
        }
    }
    tree
}

pub fn run(input: &str) -> String {
    let input = input.split('\n').map(|s| s.trim_end()).collect::<Vec<_>>();
    let part1 = part1(&input);
    let part2 = part2(&input);
    format!(
        "Part 1: The sum of small directories is {part1}\n\nPart 2: Smallest directory to delete is {part2} bytes"
    )
}

fn part1(input: &[&str]) -> u64 {
    let tree = parse_to_tree(input);
    let mut sizes = Vec::new();
    let _total_size = tree.find_sizes(None, &mut sizes);
    sizes
        .into_iter()
        .filter(|&size| size <= 100000)
        .sum::<u64>()
}

fn part2(input: &[&str]) -> u64 {
    let tree = parse_to_tree(input);
    let mut sizes = Vec::new();
    let total_size = tree.find_sizes(None, &mut sizes);
    let unused_space = 70000000 - total_size;
    sizes
        .into_iter()
        .filter(|&size| unused_space + size >= 30000000)
        .min()
        .unwrap()
}
