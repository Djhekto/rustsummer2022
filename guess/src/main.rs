use std::io; 
use std::cmp::Ordering;
use rand::Rng;


fn main() 
{
    println!("Guess the number!");

    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    println!("The secret number is: {secret_number}");
    println!("You guessed: {guess} ");
  // println!("You guessed: {guess} test     --{guess}{guess}{guess}{guess}{guess}");
  //  print!("You guessed: { } { } { }",guess,guess,guess);
  let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
//  let guess: u32 = guess.trim().parse().expect("Please type a number!");
//The trim method on a String instance will eliminate any whitespace at the beginning and end
  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {println!("You win!");break; }}
        }
}
