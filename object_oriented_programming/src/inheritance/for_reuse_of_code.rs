/*
    Reuse of code can be achieved in Rust by using traits and trait bounds with default implementations.
*/

trait Article {
    fn author(&self) -> String;
    fn content(&self) -> String;
    fn summary(&self) -> String {
        format!("{} by {}", self.content(), self.author())
    }
}

struct NewsArticle {
    author: String,
    content: String,
}

impl NewsArticle {
    fn new(author: String, content: String) -> Self {
        Self { author, content }
    }
}

impl Article for NewsArticle {
    fn author(&self) -> String {
        self.author.clone()
    }

    fn content(&self) -> String {
        self.content.clone()
    }
}

struct CoolNewsArticle {
    author: String,
    content: String,
}

impl CoolNewsArticle {
    fn new(author: String, content: String) -> Self {
        Self { author, content }
    }
}

impl Article for CoolNewsArticle {
    fn author(&self) -> String {
        self.author.clone()
    }

    fn content(&self) -> String {
        self.content.clone()
    }
}

pub fn run() {
    let news_article = NewsArticle::new(
        String::from("John Doe"),
        String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
    );

    let cool_news_article = CoolNewsArticle::new(
        String::from("Jane Doe"),
        String::from("Wowem ipsum dowow sit amet, ^•ﻌ•^ consectetuw a-adipiscing e-ewit."),
    );

    println!("News article summary: {}", news_article.summary());
    println!("Cool news article summary: {}", cool_news_article.summary());
}
