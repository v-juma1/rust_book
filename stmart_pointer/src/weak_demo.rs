#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{
    borrow::Borrow,
    cell::RefCell,
    rc::{Rc, Weak},
};
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),

        //branch的children指向leaf
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    //leaf的parent指向branch
    //互有指向，但没有循环应用
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf: {:?}", leaf.parent.borrow().upgrade());
}
