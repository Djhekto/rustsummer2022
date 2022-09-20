//[profile.release]
//panic = 'abort'  //men1she razmer exe

#![allow(unused)]

fn main() {
 //   panic!("crash and burn");
 enum _Result<T, E> { //<type / error>
    Ok(T),
    Err(E),
}
use std::fs::File;

/*
 let f = File::open("hello.txt");
 let f = match f {
     Ok(file) => file,
     Err(error) => panic!("Problem opening the file: {:?}", error),
 };
*/
/*
use std::io::ErrorKind;

let f = File::open("hello.txt");
let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt")                   {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),             },
        other_error => {panic!("Problem opening the file: {:?}", other_error)}
                                       },
            };
*/

    let _f = File::open("hello.txt").unwrap();

    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
    //     return err
//use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // vmesto !panic vernuti => vne func reshaem
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
}