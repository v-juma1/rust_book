#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, thread, time::Duration};
pub fn main() {
    let a = 10;
    let b = 6;
    really_work(a, b);

    move_test();
}

//测试move关键字的函数
fn move_test() {
    let x = vec![1, 2, 3];

    //闭包可以访问定义它的作用域内的变量，而普通函数则不能
    let equal_to1 = |z: Vec<i32>| z == x;

    //在参数列表前使用move关键字，可以强制闭包取得它所使用的环境值的所有权
    let equal_to2 = move |z: Vec<i32>| z == x;

    //下面的代码报错，x在当前作用域已经失去所有权
    //println!("{:#?}", x)
}

//声明strcut，用来保存复杂的函数和它的运行结果
//只在需要结果的时候运行该闭包，并将结果保存
struct Casher<T>
where
    //T类型为一个实现了如下trait的闭包（匿名函数）
    T: Fn(u32) -> u32,
{
    caculate: T,

    //传不同的参数给expensive_caculate，返回的结果应该和参数一一对应，所以使用HashMap
    value: HashMap<u32, u32>,
}

impl<T> Casher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(caculate_fun: T) -> Casher<T> {
        Casher {
            caculate: caculate_fun,
            value: HashMap::new(),
        }
    }

    fn get_value(&mut self, arg: u32) -> u32 {
        //从HashMap中获取对应参数的计算结果
        let value = self.value.get(&arg);
        match value {
            //如果有计算结果，则将结果返回
            Some(v) => *v,

            //如果没有计算结果，则调用闭包进行计算，并将结果更新到HashMap中
            None => {
                let v = (self.caculate)(arg);
                self.value.entry(arg).or_insert(v);
                v
            }
        }
    }
}

//假设这里有一个复杂的函数，每次调用很消耗时间和资源
fn expensive_caculate(a: u32) -> u32 {
    println!("cacuating ..........");
    thread::sleep(Duration::from_secs(3));
    a
}

fn really_work(a: u32, b: u32) {
    //声明一个闭包，只在需要结果的地方调用expensive_caculate函数
    let closure_caculate = |a| {
        println!("cacuating ..........");
        thread::sleep(Duration::from_secs(3));
        a
    };
    let mut casher = Casher::new(closure_caculate);

    if a < 25 {
        //注意前两次的参数相同，应该将结果缓存，返回相同的计算结果，第三次应该重新计算，返回不同的结果
        println!("在a<25第一次运行：{}", casher.get_value(a));
        println!("在a<25第二次运行：{}", casher.get_value(a));
        println!("在a<25第三次运行：{}", casher.get_value(a + 1));
    } else {
        if b == 3 {
            println!("不需要运行");
        } else {
            //注意前两次的参数相同，应该将结果缓存，返回相同的计算结果，第三次应该重新计算，返回不同的结果
            println!("在a>=25对的情况下运行：{}", casher.get_value(a + 2));
            println!("在a>=25对的情况下运行：{}", casher.get_value(a + 2));
            println!("在a>=25对的情况下运行：{}", casher.get_value(a + 3));
        }
    }
}
