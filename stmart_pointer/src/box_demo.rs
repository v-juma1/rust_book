#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ops::Deref;

//使用Box<T>存储数据的引用，引用即指向heap的地址具有固定大小
//因此能够计算出递归数据的大小
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//实现deref trait才能使mybox具有解引用的功能
//注意deref方法的返回值,&Self::Target在本例子中是&T
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn say_hi(name: &str) {
    println!("Hi,{}", name);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list() {
        let l1 = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
    }

    #[test]
    fn test_mybox() {
        //因为实现了eref trait 所以可以使用*进行解引用操作
        //*x等价于*(x.deref()),后面这部分由rust操作
        let x = MyBox::new(3);
        assert_eq!(3, *x);
    }

    //隐式解引用转化
    //把T的引用转化为T经过deref后的引用
    #[test]
    fn test_deref_coercion() {
        let x = MyBox::new(String::from("jack"));
        say_hi(&x);
    }
}
