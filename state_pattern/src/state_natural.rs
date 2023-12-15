/*
    Another, more natural way to implement states and transitions is to encode states as types.
    This way, calling an unavailable method on a state will result in a compile error.
    We can also only transition to a next state that is available for the current state.
*/

pub struct Post {}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost::new()
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

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
    pub fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }
}

pub struct PublishedPost {
    content: String,
}

impl PublishedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub fn run() {
    println!("Creating a new post");
    let mut post = Post::new();

    println!("Adding text to the post");
    post.add_text("I ate a salad for lunch today.");

    println!("Requesting a review");
    let post = post.request_review();

    println!("Approving the post");
    let post = post.approve();

    println!("Publishing the post");
    println!("Post content: {}", post.content());
}

/*
    As you can see, this also has some drawbacks.
    For example, we cannot implement a default behaviour for all states.
    The Post types are also not interchangeable.
    If we want to work with a finished and approved post, we have to use the PublishedPost type.
    Using the Post type results in having a Post that is neither finished nor approved.
*/
