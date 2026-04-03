pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{} by {} (at {})",
            self.headline, self.author, self.location
        )
    }
}

pub enum PostType {
    Original,
    Reply { from_user: String },
    Repost { from_user: String },
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub post_type: PostType,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        match &self.post_type {
            PostType::Original => {
                format!("{}: {}", Self::account(&self.username), self.content)
            }
            PostType::Reply { from_user } => {
                format!("Reply from {}: {}", Self::account(from_user), self.content)
            }
            PostType::Repost { from_user } => {
                format!("Repost from {}: {}", Self::account(from_user), self.content)
            }
        }
    }
}

impl SocialPost {
    pub fn account(username: &String) -> String {
        format!("@{}", username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news!");
    println!("{}", item.summarize());
    println!();
}
