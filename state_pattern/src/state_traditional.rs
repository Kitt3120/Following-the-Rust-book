/*
    We want to have posts.
    A post can hold some State or None.
    A post also has some content.
    When initializing a post, we want it to be in draft state and contain an empty string.
    Because the state field is private, there is no way around than creating a new post as a draft.
*/
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft::new())),
            content: String::new(),
        }
    }

    /*
        This behaviour does not depend on the state the post is in.
        Thus, it is implemented directly on the Post struct.

        If one would implement this in the State trait, it would be necessary write the same implementation for each state.
        This is an indication that this behaviour does not belong to the state.
    */
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // In addition to methods that do not depend on the state, we also implement methods that change the state on the Post struct.
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }

    // And we specifically implement methods that depend on the state on the Post struct.
    pub fn content(&self) -> &str {
        let state_ref = self.state.as_ref();

        match state_ref {
            Some(state) => state.content(self),
            None => "",
        }
    }
}

/*
    A state is defined by a trait.
    For each available state, we will implement this trait.
    Behaviour defined in the State trait will be shared by all available states.
*/
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // As content() is the same for all states except Published, we can implement a default behaviour.
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

/*
    Here we implement the different states and their behaviours to switch between states.

    Note that the parameter of each of these methods is declared as "self: Box<Self>" instead of just self.
    Thus, these methods are only valid when called on a Box holding a State.
    The methods also take ownership of the previous Box, invalidating the previous state.

    This is where the Optional type comes in handy, as declared in the Post struct.
    While changing states, for a brief moment, the post will hold a None value as its state.
    By doing this, it is ensured that any reference to the old state will also be invalid.

    The rust compiler enforces this behaviour.
    Try changing the signature to just "self" and see what happens.
*/
struct Draft {}

impl Draft {
    fn new() -> Draft {
        Draft {}
    }
}

impl State for Draft {
    // Drafts can be requested for review.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    // Drafts cannot be approved directly.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl PendingReview {
    fn new() -> PendingReview {
        PendingReview {}
    }
}

impl State for PendingReview {
    // Requesting a review on a pending review leads to no change.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Pending reviews can be approved.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published::new())
    }
}

struct Published {}

impl Published {
    fn new() -> Published {
        Published {}
    }
}

impl State for Published {
    // Published posts cannot be requested for review.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Published posts cannot be approved.
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Published posts have content.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

pub fn run() {
    println!("Creating a new post");
    let mut post = Post::new();

    println!("Adding text to the post");
    post.add_text("I ate a salad for lunch today.");
    println!("Content: {}", post.content());

    println!("Adding some more text to the post");
    post.add_text(" It was delicious.");
    println!("Content: {}", post.content());

    println!("Requesting a review");
    post.request_review();
    println!("Content: {}", post.content());

    println!("Approving the post");
    post.approve();
    println!("Content: {}", post.content());
}
