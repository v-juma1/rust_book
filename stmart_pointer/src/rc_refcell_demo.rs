#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{cell::RefCell, rc::Rc};

//将Rc<T>和RefCell<T>结合使用，实现一个拥有多重所有权的可变数据
//Rc<T> 多个所有者持有同一个数据
//RefCell<T> 可修改不可变借用的数据

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::rc_refcell_demo::List::{Cons, Nil};
pub fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(8)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a:{:?}", a);
    println!("b:{:?}", b);
    println!("c:{:?}", c);
}
