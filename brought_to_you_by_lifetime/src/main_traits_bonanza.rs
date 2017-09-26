// Traits
mod summary {
    // Traits are similar to interfaces in other languages
    // with a few differences.
    pub trait Summarizable {
        fn summary(&self) -> String {
            String::from("(Read more...)")
        }
    }
}

mod news_article {
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
}
mod tweets {
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
}

impl summary::Summarizable for news_article::NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl summary::Summarizable for tweets::Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl summary::Summarizable for String {
    fn summary(&self) -> String {
        let len = self.len();
        let max_len = 60;
        if len < max_len {
            String::from(&self[..len])
        } else {
            format!("{}...", &self[..max_len - 3])
        }
    }
}

fn summarize<T: summary::Summarizable>(item: &T) -> String {
    item.summary()
}

struct Hay {
    value: String,
}

impl summary::Summarizable for Hay {}

fn main() {
    let tweet = tweets::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("Tweet: {}", summarize(&tweet));
    let quote = String::from(
        "Let us look through the bends again. Let us look through the bends again.",
    );
    println!("String: {}", summarize(&quote));
    let quote = String::from(
        "Hello, my friend, we meet again. Let us look through the bends again. Let us look through the bends again.",
    );
    println!("String: {}", summarize(&quote));
    let hay = Hay { value: String::from("hay hay hay") };
    println!("Hay: {}", summarize(&hay));
}