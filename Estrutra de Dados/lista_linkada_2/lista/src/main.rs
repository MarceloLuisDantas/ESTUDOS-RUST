// use std::fmt;

#[derive(Debug, Clone)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node { value: value, next: None }
    }
    pub fn show(&self) {
        print!("{} ", self.value);
        match &self.next {
            Some(node) => node.show(),
            None => println!(""),
        }
    }
    pub fn push(&mut self, value: i32) {
        match &mut self.next {
            Some(node) => node.push(value),
            None => {
                let new_node = Box::new(Node::new(value));
                self.next = Some(new_node);
            }
        }
    }
    pub fn pop(&mut self) {
        let mut next_node = self.next.clone().unwrap();
        match next_node.next {
            Some(_) => { 
                match &mut self.next {
                    Some(next) => next.pop(),
                    None => {}
                }
            },
            None => { 
                self.next = None
            },
        }
    }
}

pub struct Lista {
    len: usize,
    head: Node,
}

impl Lista {
    pub fn new(nun: i32) -> Self {
        Lista {len: 0, head: Node{value: nun, next: None}}
    }
    pub fn show(&self) {
        self.head.show();
    }
    pub fn push(&mut self, value: i32) {
        self.head.push(value);
        self.len += 1;
    }
    pub fn pop(&mut self) {
        self.head.pop();
        self.len -= 1;
    }
}

fn main() {
    let mut lista = Lista::new(1);
    lista.push(17);
    lista.push(24);
    lista.push(34);
    lista.push(14);
    lista.push(23);
    lista.show();
    lista.pop();
    lista.pop();
    lista.show();
    lista.push(27);
    lista.push(12);
    lista.show();
}