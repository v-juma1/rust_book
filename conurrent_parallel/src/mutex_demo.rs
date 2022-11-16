#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    sync::{Arc, Mutex},
    thread,
};
pub fn main() {
    //Arc,并发场景下的多重所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            //每个线程都获取互斥锁Mutex，并给counter+1
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle)
    }

    //等所有线程都执行完成，再继续
    for handle in handles {
        handle.join().unwrap();
    }

    let res = counter.lock().unwrap();
    println!("{}", *res)
}

fn show_mutex() {
    //mutex 互斥锁：在同一时刻，只允许一个线程访问数据

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }

    println!("{:?}", m);
}
