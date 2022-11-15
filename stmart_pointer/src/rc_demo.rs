#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    //两个list共享一个共同的list
    //第一个lsit: 2 +共享的3,4 list
    //第二个lsit: 5 +共享的3,4 list
    #[test]
    fn test_list() {
        let common_l = Rc::new(List::Cons(3, Rc::new(List::Cons(4, Rc::new(List::Nil)))));
        let l1 = List::Cons(2, Rc::clone(&common_l));
        let l2 = List::Cons(5, Rc::clone(&common_l));
    }
}
