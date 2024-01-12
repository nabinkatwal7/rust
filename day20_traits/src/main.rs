pub trait Summary{
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        String::from("(Read more from ...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String{
        self.author.clone()
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

}

// impl Summary for NewsArticle {}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

}

pub fn notify<T:Summary>(item: &T){
    println!("Breaking news! {}", item.summarize());
}

fn main(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the bestockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());
}