#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{thread, time::Duration};

pub fn main() {
    let v = vec![1, 2, 3];

    //子线程执行
    //使用move关键字在线程间移交数据所有权
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        for i in 1..10 {
            println!("子线程输出：{}", i);
            println!("子线程输出：{:?}", v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //主线程执行
    for i in 1..5 {
        println!("主线程输出：{}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //join()方法会阻止当前线程（主线程）的运行，直到handle对应的线程结束运行
    handle.join().unwrap();
}
