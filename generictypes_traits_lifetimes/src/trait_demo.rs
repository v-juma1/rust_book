#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::{self, Debug, Display};

//定义trait
pub trait Summary {
    //trait 中的函数没有函数体
    fn summarize(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
struct Node;

//通过derive的方式添加注解需要注意
//跟上面的struct不同，因为Tweet的字段有String类型
//String类型没有实现Copy trait
//所以Tweet不能通过derive添加Copy
#[derive(Debug, Clone)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 实现Summary
impl Summary for Tweet {
    //实现traitz中的方法
    fn summarize(&self) -> String {
        format!("Summary:{}__{}", self.content, self.username)
    }
}

// 实现display
impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n{{username:{} \ncontent:{} \nreply:{} \nretweet:{}}}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

//trait作为函数的形参
//x为任意实现了Summary trait的类型
//使用impl语法糖
fn show_summary_1(x: &impl Summary) {
    println!("summary_1:{}", x.summarize());
}

//使用泛型特征
fn show_summary_2<T: Summary>(x: &T) {
    println!("summary_2:{}", x.summarize());
}

//指定多个trait bound，可以使用+号链接
fn show_summary_3(x: &(impl Summary + Display)) {
    println!("summary_3:{}", x.summarize());
}

//使用泛型特征
fn show_summary_4<T: Summary + Display>(x: &T) {
    println!("summary_4:{}", x.summarize());
}

//多个trait bound可以用where子句改写，使函数签名不至太长
//下面的函数，签名过长
fn notify_1<T: Summary + Display, U: Clone + Debug>(a: &T, b: &U) -> String {
    let s = format!("notify_1:{}", a.summarize());
    println!("{}", &s);
    s
}

//改用where子句
fn notify_2<T, U>(a: &T, b: &U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    let s = format!("notify_2:{}", a.summarize());
    println!("{}", &s);
    s
}

//可以指定trait bound作为函数返回类型
//但是函数只能返回确定的同一种类型否则报错
fn notify_test<T: Summary + Display + Clone>(a: T) -> impl Summary {
    println!("notify_test:{}", a);
    a
}

pub fn main() {
    let t = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let n = Node;

    show_summary_1(&t);
    show_summary_2(&t);
    show_summary_3(&t);
    show_summary_4(&t);
    notify_1(&t, &n);
    notify_2(&t, &n);

    notify_test(t);
}
