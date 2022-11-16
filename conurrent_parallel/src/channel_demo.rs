#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{sync::mpsc, thread, time::Duration};

//消息传递的方式进行线程间通信的例子
//mpsc:多个生产者，一个消费者
pub fn main() {
    let (tx, rx) = mpsc::channel();

    //通过clone创建多个发送者
    let tx_clone = mpsc::Sender::clone(&tx);

    //第一个发送者
    thread::spawn(move || {
        let v = vec![
            String::from("tx: hello"),
            String::from("tx: every one"),
            String::from("tx: bulabula"),
            String::from("tx: !"),
        ];

        for s in v {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //第二个发送者
    thread::spawn(move || {
        let v = vec![
            String::from("tx_clone: hello"),
            String::from("tx_clone: every one"),
            String::from("tx_clone: bulabula"),
            String::from("tx_clone: !"),
        ];

        for s in v {
            tx_clone.send(s).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        println!("收到：{}", r);
    }
}
