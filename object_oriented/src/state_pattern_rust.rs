#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

///以博文发布为例子，按照rust的方式实现状态模式

//正式发布的博文struct
struct Post {
    content: String,
}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    fn content(&self) -> &str {
        &self.content
    }
}

//草稿的博文struct
struct DraftPost {
    content: String,
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

//待审批的博文struct
struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

pub fn main() {
    let mut post = Post::new();
    post.add_text("按照rust的方式实现状态模式!");
    let post = post.request_review();
    let post = post.approve();
    println!("博文内容：{}", post.content());
}
