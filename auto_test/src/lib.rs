pub fn add(left: i32, right: i32) -> i32 {
    if left > 0 && right > 0 {
        left + right
    } else if left == 0 && right == 0 {
        panic!("请传入非0的数")
    } else {
        panic!("请传入大于0的数")
    }
}

//默认的运行方式，多线程并行运行
//cargo test -p auto_test --lib

//指定线程数，单线程运行
//cargo test -p auto_test --lib -- --test-threads=1

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]标识测试函数
    #[test]
    fn assert_eq_works() {
        let result = add(2, 2);

        //添加自定义信息
        assert_eq!(result, 4, "{}和{}不相等", result, 4);
    }

    #[test]
    fn assert_ne_works() {
        let result = add(2, 2);

        //添加自定义信息
        assert_ne!(result, 5, "{}和{}相等", result, 4);
    }

    //测试 #[should_panic]
    //程序只有触发#panic 测试才会通过，否则不能通过
    #[test]
    #[should_panic]
    fn panic_works() {
        let result = add(-1, -1);
    }

    //测试 #[should_panic]
    //程序只有触发包含指定信息的panic,测试才会通过，否则不能通过
    #[test]
    #[should_panic(expected = "请传入非0的数")]
    fn certain_panic_works() {
        let result = add(0, 0);
    }

    //使用Result类型代替assert断言
    #[test]
    fn test_result() -> Result<(), String> {
        if 4 == add(2, 2) {
            Ok(())
        } else {
            Err(String::from("4 == add(2, 2) 不成立"))
        }
    }
}
