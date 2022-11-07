#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{Debug, Display};

//定义trait
pub trait Summary {
    //trait 中的函数没有函数体
    fn summarize(&self) -> String;

    //也可以给定具体的函数作为默认实现
    fn sum_test(&self) -> String {
        String::from("default fuction test")
    }

    //也可以给定具体的函数作为默认实现
    fn sum_demo(&self) -> String {
        String::from("default Summary")
    }
}

//定义两个struct
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//为两个结构体实现trait
impl Summary for NewsArticle {
    //实现traitz中的方法
    fn summarize(&self) -> String {
        format!("Summary:{},{}", self.headline, self.author)
    }
}

impl Summary for Tweet {
    //实现traitz中的方法
    fn summarize(&self) -> String {
        format!("Summary:{},{}", self.content, self.username)
    }
    //重载trait中已有的方法
    //可以调用tait中本身之外其他已经实现的方法
    //调用本身self.sum_test()编译也能通过
    //但显然是死循环，以fatal runtime error: stack overflow结束
    fn sum_test(&self) -> String {
        format!("Tweet test:{}", self.sum_demo())
    }
}

//trait作为函数的形参
//x为任意实现了Summary trait的类型
fn show_summary(x: &impl Summary) {
    println!("{}", x.summarize());
}

//也可以使用trait bound 语法实现上面的函数
fn show_summary_2<T: Summary>(x: &T) {
    println!("{}", x.summarize());
}

//指定多个trait bound，可以使用+号链接
fn show_summary_3(x: &(impl Summary + Display)) {
    println!("{}", x.summarize());
}
fn show_summary_4<T: Summary + Display>(x: &T) {
    println!("{}", x.summarize());
}

//多个trait bound可以用where子句改写，使函数签名不至太长
//下面的函数，签名过长
fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("{}", a.summarize())
}

//可以指定trait bound作为函数返回类型
//但是函数只能返回确定的同一种类型否则报错
fn notify_test<T: Summary>(a: T) -> impl Summary {
    a
}

//改用where子句
fn notify2<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("{}", a.summarize())
}

pub fn main() {
    let n1 = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    let t1 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("{}", n1.summarize());
    println!("{}", t1.summarize());

    //调用默认实现
    println!("{}", n1.sum_test());
    println!("{}", t1.sum_test());

    show_summary(&n1);
    show_summary_2(&n1);
}
