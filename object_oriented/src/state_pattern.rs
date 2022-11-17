#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

///以博文发布为例子，按照面向对象的方式实现状态模式

//状态trait
trait State {
    //请求审批
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    //审批通过
    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

//草稿
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReiew {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

//等待审批
struct PendingReiew {}

impl State for PendingReiew {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

//已发布
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

//草稿，等待审批，已发布三种状态中的一种
//用户通过定义在post中的方法改变状态
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    fn request_review(&mut self) {
        //获取状态
        if let Some(s) = self.state.take() {
            //更新状态
            self.state = Some(s.request_review());
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

pub fn main() {
    let mut post = Post::new();

    post.add_text("今天是个beautiful的day");
    println!("博文内容：{}", post.content());

    post.request_review();
    println!("博文内容：{}", post.content());

    post.approve();
    println!("博文内容：{}", post.content());
}
