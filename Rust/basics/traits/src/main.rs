pub struct Article {
    pub headline: String,
    pub author: String,
    pub word_count: i32,
    pub content: String,
}

pub struct Tweet {
    pub author: String,
    pub char_count: i32,
    pub edited: bool,
    pub content: String,
}

impl Summary for Article {
    fn get_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.headline)
    }
}

impl Summary for Tweet {
    fn get_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

pub trait Summary {
    fn get_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}", self.get_author())
    }
}

fn main() {
    let tweet = Tweet {
        author: String::from("Dipto"),
        char_count: 355,
        edited: false,
        content: String::from("Today is a rainy day!"),
    };

    let article = Article {
        headline: String::from("Sunny Day"),
        author: String::from("Arya"),
        word_count: 55,
        content: String::from("Today is a sunny day!"),
    };
    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
    println!("Tweet author: {}", article.get_author());
}
