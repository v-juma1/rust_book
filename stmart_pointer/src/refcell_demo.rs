#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percent = self.value as f64 / self.max as f64;
        if percent > 1.0 {
            self.messenger.send("overflow,大于100%")
        } else if percent > 0.9 {
            self.messenger.send("Error,大于90%")
        } else if percent > 0.75 {
            self.messenger.send("warning,大于75%")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            //borrow_mut方法获取内部值的可变引用，从而修改不可变的变量
            //borrow_mut方法返回智能指针RefMut<T>
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn test_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        //borrow方法获取内部值的不可变引用
        //borrow方法返回智能指针Ref<T>
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
