#![allow(unused)]

enum Message { //bez  struct  cnhbyu  tuple
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//vmesto
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message { //metod enum
    fn call(&self) {
        // method body would be defined here
    }
}
//togda
/*    let m = Message::Write(String::from("hello"));
m.call();
}
*/
fn main() {
    println!("Hello, world!");
}
/*
enum Option<T> {
    None,
    Some(T),
}
 */