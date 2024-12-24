pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

//impl Post {
//    pub fn new() -> Post {
//        Post {
//            state: Some(Box::new(Draft {})),
//            content: String::new(),
//        }
//    }
//
//    pub fn request_review(&mut self) {
//        if let Some(s) = self.state.take() {
//            self.state = Some(s.request_review())
//        }
//    }
//
//    pub fn add_text(&mut self, text: &str) {
//        self.content.push_str(text);
//    }
//
//    pub fn approve(&mut self) {
//        if let Some(s) = self.state.take() {
//            self.state = Some(s.approve())
//        }
//    }
//
//    pub fn content(&self) -> &str {
//        self.state.as_ref().unwrap().content(self)
//    }
//}
//
//trait State {
//    fn request_review(self: Box<Self>) -> Box<dyn State>;
//    fn approve(self: Box<Self>) -> Box<dyn State>;
//    fn content<'a>(&self, post: &'a Post) -> &'a str {
//        ""
//    }
//}
//
//struct Draft {}
//
//impl State for Draft {
//    fn request_review(self: Box<Self>) -> Box<dyn State> {
//        Box::new(PendingReview {})
//    }
//
//    fn approve(self: Box<Self>) -> Box<dyn State> {
//        self
//    }
//}
//
//struct PendingReview {}
//
//impl State for PendingReview {
//    fn request_review(self: Box<Self>) -> Box<dyn State> {
//        self
//    }
//
//    fn approce(self: Box<Self>) -> Box<dyn State> {
//        Box::new(Published {})
//    }
//}
//
//struct Published {}
//
//impl State for Published {
//    fn request_review(self: Box<Self>) -> Box<dyn State> {
//        self
//    }
//
//    fn approve(self: Box<Self>) -> Box<dyn State> {
//        self
//    }
//
//    fn content<'a>(&self, post: &'a Post) -> &'a str {
//        &post.content
//    }
//}

// Take 2 with a more native to Rust implementation of this pattern

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            cotent: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
