#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

//Struct中使用泛型
#[derive(Debug)]
struct Point<T, E> {
    x: T,
    y: E,
}
//函数中使用泛型
//对所有泛型实现的方法，对每一种具体的泛型类型都有效
impl<T, E> Point<T, E> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

//可以只针对某一具体的类型实现struct函数，只对对应的类型有效
impl Point<i32, f64> {
    fn get_x_i32(&self) -> &i32 {
        &self.x
    }
}

//struct 泛型类型参数可以和方法的类型参数不同
//下面函数将两个不同类型的p混合，取一个的x，另一个的y
impl<T, E> Point<T, E> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//所有类型的Point struct都有下面的方法
//self:表示调用函数的实例本身（调用者）
//Self:调用实例本身的类型（调用者类型）
impl<T, E> Point<T, E> {
    fn new(x: T, y: E) -> Self {
        Self { x, y }
    }
}

//有条件地为实现了特定trait的Point实现方法
//即只有实现了T和E对应 trait的Point<T, E>类型才能调用cmp_display方法
impl<T> Point<T, T>
where
    T: Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest is x={}", self.x);
        } else {
            println!("the largest is y={}", self.y);
        }
    }
}

pub fn main() {
    let p = Point { x: 5, y: 6.7 };
    println!("{:#?}", p);
    println!("{}", p.get_x());
    println!("{}", p.get_x_i32());

    let p2 = Point { x: "a", y: "b" };
    let p3 = p2.mixup(p);
    println!("{:#?}", p3);
}
