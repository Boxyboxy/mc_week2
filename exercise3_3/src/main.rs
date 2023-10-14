// create a struct Novel with title, author, genre (all String)

struct Novel {
    title: String,
    author: String,
    genre: String,
}

// create a struct NonFiction with title, author, topic (all String)
struct NonFiction {
    title: String,
    author: String,
    topic: String,
}

fn print_book_summary<T:Book>(x:T){
    x.get_summary();
}

// create a trait Book
trait Book {
    fn get_summary(&self);
}
// implement Book trait for Novel
// define get_summary which takes in &self
// prints "<Book Title> is a <Book Genre> written by <Book Author>"
impl Book for Novel {
    fn get_summary(&self) {
        println!("{} is a {} written by {}", self.title, self.genre, self.author);
    }
}
// implement Book trait for NonFiction
// define get_summary which takes in &self
// prints "<Book Title> is a <Book Topic> written by <Book Author>"
impl Book for NonFiction {
    fn get_summary(&self) {
        println!("{} is a {} written by {}", self.title, self.topic, self.author);
    }
}
// define fuction wihch takes in a generic that that implements Book trait
// call get_summary with the book
fn main() {
// create book_1 of instance Novel
let book_1 = Novel {
    title: "Kafka by the shore".to_owned(),
    author: "Haruki Murakamai".to_owned(),
    genre: "Fantasy Fiction".to_owned(),
};
// create book_2 of instance NonFiction
let book_2 = NonFiction {
    title: "Winner takes all".to_owned(),
    author: "Anand Giridharadas".to_owned(),
    topic: "Economic Inequality".to_owned(),
};
    // call print_book_summary with book_1
    book_1.get_summary();
    // call print_book_summary with book_2  
    book_2.get_summary();
    print_book_summary(book_2);
}