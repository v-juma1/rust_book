#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

//rust 的struct方法，使用impl关键字声明
//这些函数也可拆分到不同的impl块中
impl Rectangle {
    //struct方法的第一个参数self，代表实例本身
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    //关联函数，类似于java,C#等语言的类方法，一般用来生成struct实例
    //关联函数第一个参数不是self，区别于struct方法
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

pub fn run_demo() {
    let rec = Rectangle {
        length: 50,
        width: 30,
    };

    //关联函数的调用方法
    let small_square = Rectangle::square(20);

    //虽然area函数的参数是Rectangle的引用，但rust可以自动解引用
    //所以下面的写法等价于：let s = (&rec).area();
    let s = rec.area();
    let is_hold = rec.can_hold(&small_square);

    println!("{:#?}", rec);
    println!("{}", s);
    println!("can hold:{}", is_hold)
}
