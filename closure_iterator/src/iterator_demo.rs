#![allow(dead_code)]
#![allow(unused_variables)]
//实现自定义迭代器的例子
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();

    for item in v1_iter {
        println!("{}", item)
    }

    let v2_iter = v1.iter();
    let total: i32 = v2_iter.sum();
    println!("sum:{}", total);

    //惰性的，调用消耗性方法，才能生效
    let m1 = v1.iter().map(|&x| x + 1);
    let v2: Vec<i32> = m1.collect();
    println!("v2:{:#?}", v2);

    //注意filter 和上面map的细微区别
    let m2 = v1.iter().filter(|&&x| x % 2 == 0);
    let v3: Vec<&i32> = m2.collect();
    println!("v3:{:#?}", v3);
}

#[test]
fn test_iter() {
    let mut c = Counter::new();
    assert_eq!(Some(1), c.next());
    assert_eq!(Some(2), c.next());
    assert_eq!(Some(3), c.next());
    assert_eq!(Some(4), c.next());
    assert_eq!(Some(5), c.next());
    assert_eq!(None, c.next());
}
