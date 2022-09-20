#![allow(unused)]

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { return x } 
    else { return y }
                                                  }
//
#[derive(Debug)] struct ImportantExcerpt<'a> {
    part: &'a str,
}
//---------------------------------------------------//
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>
    ( x: &'a str, y: &'a str, ann: T,) -> &'a str
        where T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {   x} 
    else { y }
}
//---------------------------------------------------//
fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("lol");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence,};
    println!("The longest string is {:?}", i);

let s: &'static str = "I have a static lifetime.";

let string1 = String::from("abcd");
let string2 = "xyz";
let result = longest_with_an_announcement( string1.as_str(), string2, "Today is someone's birthday!",);
println!("The longest string is {}", result);

}

//struct Foo1 { x: &bool }
              // ^ expected lifetime parameter
struct Foo2<'a> { x: &'a bool } // correct

//struct Bar1 { x: Foo2 }
              // ^^^^ expected lifetime parameter
struct Bar2<'a> { x: Foo2<'a> } // correct

//enum Baz1 { A(u8), B(&bool), }
                  // ^ expected lifetime parameter
enum Baz2<'a> { A(u8), B(&'a bool), } // correct

//type MyStr1 = &str;
           // ^ expected lifetime parameter
type MyStr2<'a> = &'a str; // correct

/*&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime */