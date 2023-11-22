/*
    Inheritance is not a feature of Rust.
    However, using dynamic dispatch, we can still achieve polymorphism.

    From the book:
    To many people, polymorphism is synonymous with inheritance.
    But it’s actually a more general concept that refers to code that can work with data of multiple types.
    For inheritance, those types are generally subclasses.

    Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide.
    This is sometimes called bounded parametric polymorphism.

    Inheritance has recently fallen out of favor as a programming design solution in many programming languages
    because it’s often at risk of sharing more code than necessary.
    Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance.
    This can make a program’s design less flexible.
    It also introduces the possibility of calling methods on subclasses that don’t make sense or that cause errors
    because the methods don’t apply to the subclass.
    In addition, some languages will only allow single inheritance (meaning a subclass can only inherit from one class),
    further restricting the flexibility of a program’s design.

    For these reasons, Rust takes the different approach of using trait objects instead of inheritance.
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
    pages: u32,
}

impl CoolNewsArticle {
    fn new(author: String, content: String, pages: u32) -> Self {
        Self {
            author,
            content,
            pages,
        }
    }

    fn pages(&self) -> u32 {
        self.pages
    }
}

impl Article for CoolNewsArticle {
    fn author(&self) -> String {
        self.author.clone()
    }

    fn content(&self) -> String {
        self.content.clone()
    }

    // Override default implementation and use late binding. This is called dynamic dispatch in Rust and comes with a performance penalty at runtime.
    fn summary(&self) -> String {
        format!(
            "{} by {} ({} pages)",
            self.content(),
            self.author(),
            self.pages()
        )
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
        10,
    );

    // To use traits as types and struct instances as trait objects, we have to use a smart pointer, like Box<T>, in combination with dynamic dispatch.
    let articles: Vec<Box<dyn Article>> = vec![Box::new(news_article), Box::new(cool_news_article)];

    for (i, article) in articles.iter().enumerate() {
        println!("Article #{}: {}", (i + 1), article.summary()); // Note how this will use the overridden summary() method for CoolNewsArticle.
    }
}
