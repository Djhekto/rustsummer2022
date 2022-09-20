pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String { format!("(Read more from {}...)", self.summarize_author()) }
                  }
//----------------------------------------------------------------------------------//
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
                 }
impl Summary for Tweet { 
    fn summarize_author(&self) -> String { format!("@{}", self.username) }
                       }
//----------------------------------------------------------------------------------//
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify1<T: Summary>(item: &T) {  //trait bound => better
    println!("Breaking news! {}", item.summarize());
}
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// =>
//pub fn notify<T: Summary>(item1: &T, item2: &T) {

//pub fn notify(item: &(impl Summary + Display)) {
// =>
//pub fn notify<T: Summary + Display>(item: &T) {
//----------------------------------------------------------------------------------//
fn returns_summarizable() -> impl Summary {
    return Tweet {
        username: String::from("hehe"),
        content: String::from(  "of course"),
        reply: false,
        retweet: false,
    }
}
//----------------------------------------------------------------------------------//
fn main() {
    let tweet = Tweet {
        username: String::from("user123213"),
        content: String::from("gregfxcb"),
        reply: false,
        retweet: false,
    };
    println!(" {}", tweet.summarize_author());
    println!(" {}", tweet.summarize());
    notify(&tweet);
    notify1(&tweet);
    returns_summarizable();
}