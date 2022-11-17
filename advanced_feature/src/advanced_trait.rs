#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
//! trait A:B 表示A继承B，适用于在tait A中调用了trait B 的情况，要求struct实现A和B两个trait
//!
//! 孤儿规则：当trait或者struct中至少有一个定义在本地包中时，才能为struct 实现 trait
//!
//! type关键字还可以为类型指定别名
//!
//! never类型，即"!", 表示一个函数或者代码块没有返回值，never类型可以被强制转换为任意其他类型
//! 例如在Result<T> 枚举中Err分支的表达式为continue,或者Option枚举中None分支表达式为panic等情况下，该分支的返回值为never类型

///默认泛型参数
/// <T=E> 表示如果没有指定具体的泛型，则默认泛型就是E
/// 可以通过重载std::ops中的trait来重载一部分运算符
/// 一个通过重载运算符实现米和毫米相加的例子
mod first_demo {
    use std::ops::Add;
    #[derive(Debug)]
    struct Millimeter(u32);

    struct Meter(u32);

    impl Add<Meter> for Millimeter {
        type Output = Millimeter;

        fn add(self, rhs: Meter) -> Self::Output {
            Millimeter(self.0 + rhs.0 * 1000)
        }
    }
    pub fn test_add() {
        let m1 = Meter(1);
        let m2 = Millimeter(100);
        let res = m2 + m1;
        println!("{:?}", res);
    }
}

///使用完全限定语法调用同名方法
///完全限定语法可以在任何调用函数或者方法的地方使用
mod second_demo {

    trait Pilot {
        fn fly(&self);
        fn name() -> String;
    }

    trait Wizard {
        fn fly(&self);
        fn name() -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("Pilot的fly方法");
        }
        fn name() -> String {
            String::from("Pilot中的name方法")
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Wizard的fly方法");
        }

        fn name() -> String {
            String::from("Wizard中的name方法")
        }
    }

    impl Human {
        fn fly(&self) {
            println!("Human自身的fly方法");
        }
        fn name() -> String {
            String::from("Human自身的name方法")
        }
    }

    pub fn test_fully_qualified_syntax() {
        let person = Human;

        //调用struct 自身的fly方法
        person.fly();

        //Pilot可以被多个struct实现，rsut通过参数person的类型来推断出应该调用为Human实现的fly方法
        Pilot::fly(&person);

        //Wizard可以被多个struct实现，rsut通过参数person的类型来推断出应该调用为Human实现的fly方法
        Wizard::fly(&person);

        //同名的无参数函数，调用Human自身的name方法
        println!("{}", Human::name());

        //使用完全限定语法调用两个trait中的同名方法
        println!("{}", <Human as Pilot>::name());
        println!("{}", <Human as Wizard>::name());
    }
}

//使用新创建的struct绕过孤儿规则的例子
//为Vec实现Display trait的例子
mod third_demo {
    use std::fmt;
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    pub fn test_vec_display() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

pub fn main() {
    //first_demo::test_add();
    //second_demo::test_fully_qualified_syntax();

    third_demo::test_vec_display();
}
