pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Book {
    pub title: String,
    pub author: String,
}

pub struct Movie {
    pub title: String,
    pub director: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        let description = format!("Book: {} by {}", self.title, self.author);
        description
    }
}

impl Describable for Movie {
    fn describe(&self) -> String {
        let description = format!("Movie: {} directed by {}", self.title, self.director);
        description
    }
}

pub fn print_description(item: &impl Describable)(item: &T) {
    println!("{}", item.describe());
}
