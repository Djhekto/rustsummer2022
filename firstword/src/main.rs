fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();//convert our String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {  //(счетчик,буква) из всего списка букв
        if item == b' ' {  //b это байт
            return i;  //если нашли
        }
    }
    s.len() //не нашли  
}       
fn _first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {}

//A string slice is a reference to part of a String   let s123 = &s[0..5];

//Converts a string literal to a byte string literal .as_bytes(
  /*  iter(), which iterates over &T.
    iter_mut(), which iterates over &mut T.
    into_iter(), which iterates over T. */
//    An iterator that yields the current count and the element during iteration  core::iter::Enumerate 
/*
Creates an iterator which gives the current iteration count as well as the next value.
The iterator returned yields pairs (i, val), where i is the current index of iteration and val is the value returned by the iterator.
enumerate() keeps its count as a [usize]. If you want to count by a different sized integer, the zip function provides similar functionality.
*/