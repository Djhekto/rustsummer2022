#![allow(unused)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;//"?" after Result -> ok|(err + break)  vmesto raspis cerez match
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("{:?}",s); 
    Ok(s)
}
let ahah = read_username_from_file();
println!("{:?}",ahah);  //cnaruzi chitaem oshibky

fn read_username_from_file1() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
//let home: IpAddr = "127.0.0.1".parse().unwrap();

use std::fs;
fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
let hehe = last_char_of_first_line("adadada sasf\n kok op");
println!("{:?}",hehe);
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// Returns the contained [Ok] value    Panics if the value is an [Err]           .unwrap