fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
//
let condition = true;
let number = if condition { 5 } else { 6 };
println!("The value of number is: {number}");

let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
    println!("The result is {result}");

let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
/* let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
}*/


}
//If you have loops within loops, break and continue apply to the innermost loop at that point.
/*while number != 0 {
    println!("{number}!");

    number -= 1;
}*/