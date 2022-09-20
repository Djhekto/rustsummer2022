fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {  //T has 2 traits 
    let mut largest = list[0];
    for &item in list { if item > largest { largest = item; } }
    return largest
                                                  }
//------------------------------------------------------------//
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}