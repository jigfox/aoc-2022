use super::*;

type NodePointer = usize;

struct Fs {
    nodes: Vec<Node>,
    current_dir: NodePointer,
}

impl Fs {
    fn new() -> Fs {
        let dir = Directory {
            name: String::from("/"),
            children: Vec::new(),
            parent: None,
        };
        Fs {
            nodes: vec![Node::Directory(dir)],
            current_dir: 0,
        }
    }

    fn add_dir(&mut self, name: String) -> NodePointer {
        let new_dir = Node::Directory(Directory::new(name, Some(self.current_dir)));
        self.nodes.push(new_dir);
        let pointer = self.nodes.len() - 1;
        let cur_dir = &mut self.nodes[self.current_dir];
        match cur_dir {
            Node::Directory(dir) => dir.children.push(pointer),
            _ => {}
        }
        pointer
    }

    fn add_file(&mut self, size: usize) -> NodePointer {
        let new_file = Node::File(File { size });
        self.nodes.push(new_file);
        let pointer = self.nodes.len() - 1;
        let cur_dir = &mut self.nodes[self.current_dir];
        match cur_dir {
            Node::Directory(dir) => dir.children.push(pointer),
            _ => {}
        }
        self.nodes.len() - 1
    }

    fn move_up(&mut self) -> NodePointer {
        let cur_dir = &self.nodes[self.current_dir];
        match cur_dir {
            Node::Directory(dir) => {
                if let Some(parent) = dir.parent {
                    self.current_dir = parent;
                }
            }
            _ => {}
        }
        self.current_dir
    }

    fn change_dir(&mut self, name: String) -> NodePointer {
        let cur_dir = &self.nodes[self.current_dir];
        match cur_dir {
            Node::Directory(dir) => {
                for i in 0..dir.children.len() {
                    let pointer = dir.children[i];
                    let dir_iter = &self.nodes[pointer];
                    match dir_iter {
                        Node::Directory(dir2) => {
                            if dir2.name == name {
                                self.current_dir = pointer;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        self.current_dir
    }

    fn dir_lte(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .filter_map(|n| match n {
                Node::Directory(dir) => Some(dir),
                Node::File(_) => None,
            })
            .map(|a| self.dir_size(a))
            .collect()
    }

    fn dir_gte(&self, limit: usize) -> usize {
        let min = self.nodes
            .iter()
            .filter_map(|n| match n {
                Node::Directory(dir) => Some(dir),
                Node::File(_) => None,
            })
            .map(|a| self.dir_size(a)).filter(|s| s >= &limit).min();
        match min {
            Some(m) => m,
            None => 0,
        }
    }

    fn dir_size(&self, dir: &Directory) -> usize {
        dir.children
            .iter()
            .map(|p| &self.nodes[*p])
            .map(|n| match n {
                Node::Directory(dir) => self.dir_size(&dir),
                Node::File(file) => file.size,
            })
            .sum::<usize>()
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<NodePointer>,
    parent: Option<NodePointer>,
}

impl Directory {
    fn new(name: String, parent: Option<NodePointer>) -> Directory {
        Directory {
            name,
            children: Vec::new(),
            parent,
        }
    }
}

#[derive(Debug)]
enum Node {
    File(File),
    Directory(Directory),
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut fs = Fs::new();
        for line in lines {
            if let Ok(ip) = line {
                let output = ip.split(" ").collect::<Vec<_>>();
                match output[0] {
                    "dir" => {
                        fs.add_dir(output[1].to_string());
                    }
                    "$" => {
                        if output[1] == "cd" {
                            match output[2] {
                                ".." => {
                                    fs.move_up();
                                }
                                _ => {
                                    fs.change_dir(output[2].to_string());
                                }
                            }
                        }
                    }
                    _ => {
                        if let Ok(size) = output[0].parse::<usize>() {
                            fs.add_file(size);
                        }
                    }
                }
            }
        }
        println!(
            "part1: {}",
            fs.dir_lte()
                .iter()
                .filter(|s| **s <= 100000usize)
                .sum::<usize>()
        );
        let root_size = match &fs.nodes[0] {
            Node::Directory(dir) => fs.dir_size(&dir),
            _ => 0,
        };
        let to_delete = 30000000 - (70000000 - root_size);
        println!("part2: {}", fs.dir_gte(to_delete));
    }
}
