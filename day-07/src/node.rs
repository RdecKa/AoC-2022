use std::collections::HashMap;

/* Tree **********************************************************************/
#[derive(Debug)]
pub struct Tree {
    all_nodes: HashMap<String, Node>,
}

impl Tree {
    pub fn new() -> Tree {
        let root_path = "/";
        let mut nodes = HashMap::new();
        nodes.insert(
            root_path.to_string(),
            Node::Directory(Directory::new(root_path)),
        );
        Tree { all_nodes: nodes }
    }

    pub fn create_dir(&mut self, path: &str, name: &str) {
        let dir = Node::Directory(Directory::new(name));
        let full_name = path.to_string() + name + "/";

        // Add child to the parent directory
        self.all_nodes
            .entry(path.to_string())
            .and_modify(|node| match node {
                Node::Directory(d) => d.add_child(&full_name),
                Node::File(_) => panic!("This cannot be a file"),
            });

        // Save the node
        self.all_nodes.insert(full_name, dir);
    }

    pub fn create_file(&mut self, path: &str, name: &str, size: i32) {
        let file = Node::File(File::new(name, size));
        let full_name = path.to_string() + name;

        // Add child to the parent directory
        self.all_nodes
            .entry(path.to_string())
            .and_modify(|node| match node {
                Node::Directory(d) => d.add_child(&full_name),
                Node::File(_) => panic!("This cannot be a file"),
            });

        // Save the node
        self.all_nodes.insert(full_name, file);
    }

    pub fn get_node_from_path(&self, path: &str) -> Option<&Node> {
        self.all_nodes.get(path)
    }

    pub fn get_node_from_path_mut(&mut self, path: &str) -> Option<&mut Node> {
        self.all_nodes.get_mut(path)
    }

    pub fn calculate_node_sizes(&mut self, path: &str) -> i32 {
        let node = self.get_node_from_path(path).unwrap();
        let size = match node {
            Node::File(file) => file.size,
            Node::Directory(dir) => dir
                .children
                .clone()
                .iter()
                .map(|child| self.calculate_node_sizes(child))
                .sum(),
        };
        // Get the node again to solve ownership problems
        let node = self.get_node_from_path_mut(path).unwrap();
        match node {
            Node::File(_) => {}
            Node::Directory(dir) => dir.size = Some(size),
        }
        size
    }

    // Solution for star 1
    pub fn sum_small_directories(&self) -> i32 {
        self.all_nodes
            .values()
            .map(|node| {
                if let Node::Directory(Directory { size, .. }) = node {
                    if size.unwrap() > 100000 {
                        0
                    } else {
                        size.unwrap()
                    }
                } else {
                    0
                }
            })
            .sum()
    }
}

/* Node **********************************************************************/
#[derive(Debug)]
pub enum Node {
    Directory(Directory),
    File(File),
}

pub trait Name {
    fn get_name(&self) -> &str;
}

impl Name for Node {
    fn get_name(&self) -> &str {
        match &self {
            Node::Directory(dir) => &dir.name,
            Node::File(file) => &file.name,
        }
    }
}

/* Directory *****************************************************************/
#[derive(Debug)]
pub struct Directory {
    name: String,
    children: Vec<String>,
    size: Option<i32>,
}

impl Directory {
    pub fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            children: Vec::new(),
            size: None,
        }
    }

    pub fn add_child(&mut self, full_name: &str) {
        self.children.push(full_name.to_string());
    }
}

/* File **********************************************************************/
#[derive(Debug)]
pub struct File {
    name: String,
    size: i32,
}

impl File {
    pub fn new(name: &str, size: i32) -> File {
        File {
            name: name.to_string(),
            size,
        }
    }
}
