extern crate slab;
use std::rc::Rc;
use std::collections::VecDeque;

use slab::Slab;

#[derive(Debug, Clone, Copy)]
enum Color {
    RED,
    BLACK
}

#[derive(Debug)]
struct Node {
    color: Color,
    parent: Rc<Option<Node>>,
    left: Rc<Option<Node>>,
    right: Rc<Option<Node>>,
    key: i32
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Pointer(usize);
impl Pointer {
    fn null() -> Pointer {
        Pointer(!0);
    }

    fn is_null(&self) -> bool {
        *self == Pointer::null();
    }
}

struct Tree {
    nodes: Slab<Node>,
    root: Pointer
}

struct Node2 {
    color: Color,
    parent: Pointer,
    left: Pointer,
    right: Pointer,
    key: i32
}

fn insert(root: Rc<Option<Node>>, node: Node) -> Rc<Option<Node>> {
    println!("Inserting {:?} into {:?}", node, root);
    let opt: Option<Node> = match &*root {
        None => Some(node),
        Some(root_node) => {
            if node.key < root_node.key {
                Some(Node {
                    color: root_node.color,
                    parent: root_node.parent.clone(),
                    left: insert(root_node.left.clone(), node),
                    right: root_node.right.clone(),
                    key: root_node.key
                })
            } else {
                Some(Node {
                    color: root_node.color,
                    parent: root_node.parent.clone(),
                    left: root_node.left.clone(),
                    right: insert(root_node.right.clone(), node),
                    key: root_node.key
                })
            }
        }
    };

    return Rc::new(opt);

    // fn insert_recurse(root: &mut Rc<Node>, node: Node) {
    //     // if node.key < &root.key {
    //     //     // Insert left
    //     //     if let left = &node.left
    //     // }
    //     *root.left.borrow_mut() = Some(Rc::new(node));
    // }

    // match &*tree.root.borrow_mut() {
    //     None => {
    //         *tree.root.borrow_mut() = Some(Rc::new(node));
    //         return;
    //     }
    //     Some(root) => {
    //         insert_recurse(&mut root.clone(), node);
    //     }
    // }
}

fn create_node(key: i32) -> Node {
    Node {
        color: Color::BLACK,
        parent: Rc::new(None),
        left: Rc::new(None),
        right: Rc::new(None),
        key: key
    }
}





fn print_tree(root: &Rc<Option<Node>>) {
    #[derive(Debug)]
    struct NodePrintData {
        space_left: i32,
        space_right: i32,
        node_width: i32,
        node_value: i32,
        children_count: i32,
        left: Box<Option<NodePrintData>>,
        right: Box<Option<NodePrintData>>
    }

    fn digits(key: i32) -> i32 {
        let mut digits = ((key as f32).log10() as i32) + 1;
        if key < 0 {
            digits += 1;
        }
        return digits;
    }

    fn merge_node_print_data(left: NodePrintData, right: NodePrintData, node: &Node) -> NodePrintData {
        return NodePrintData {
            space_left: left.space_left + left.node_width + left.space_right,
            space_right: right.space_left + right.node_width + right.space_right,
            node_width: digits(node.key),
            node_value: node.key,
            children_count: left.children_count + right.children_count + 1,
            left: Box::new(Some(left)),
            right: Box::new(Some(right))
        };
    }

    fn node_print_data_from_tree(root: &Rc<Option<Node>>) -> NodePrintData {
        match &**root {
            Some(root_node) => {
                let left_node_print_data = node_print_data_from_tree(&root_node.left);
                let right_node_print_data = node_print_data_from_tree(&root_node.right);
                let node_print_data = merge_node_print_data(left_node_print_data, right_node_print_data, &root_node);
                return node_print_data;
            },
            None => {
                return NodePrintData {
                    space_left: 0,
                    space_right: 0,
                    node_width: 0,
                    node_value: 0,
                    children_count: 0,
                    left: Box::new(None),
                    right: Box::new(None)
                };
            }
        }
    }

    fn print_spaces(spaces: i32) {
        for i in 0..(spaces) {
            print!(" ");
        }
    }

    fn print_node_print_data(node_print_data: &NodePrintData) {
        print_spaces(node_print_data.space_left);
        print!("{}", node_print_data.node_value);
        print_spaces(node_print_data.space_right);
    }

    let node_print_data = node_print_data_from_tree(root);

    // Breadth first tree traversal to print tree

    let mut queue: VecDeque<(i32, i32, NodePrintData)> = VecDeque::with_capacity(node_print_data.children_count as usize);
    let mut current_depth = 0;

    queue.push_back((0, 0, node_print_data));

    while queue.len() > 0 {
        let (depth, parent_width, current_node) = queue.pop_front().unwrap();
        if depth > current_depth {
            print!("\n");
            current_depth = depth;
        } else {
            print_spaces(parent_width);
        }
        print_node_print_data(&current_node);
        match *current_node.left {
            Some(node_print_data) => {
                if node_print_data.node_width > 0 {
                    queue.push_back((depth + 1, current_node.node_width, node_print_data));
                }
            }
            _ => {}
        }
        match *current_node.right {
            Some(node_print_data) => {
                if node_print_data.node_width > 0 {
                    queue.push_back((depth + 1, current_node.node_width, node_print_data));
                }
            }
            _ => {}
        }
    }
    print!("\n");
}

fn main() {
    let tree: Rc<Option<Node>> = Rc::new(None);

    let node1 = create_node(10);
    let node2 = create_node(12);
    let node3 = create_node(9);
    let node4 = create_node(8);

    print_tree(&insert(insert(insert(insert(tree, node1), node2), node3), node4));
}

