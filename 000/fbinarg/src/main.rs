/* arg имя_файла номер_строки
открыть файл 256 битных беззначных бинарных чисел 
прочитать нужную строку
произвести умножение числа со строки вдвое
*/

use std::env;
//use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::File;

 /* chiclo*2 
    0->0  1->10->+0  
   +0->1 +1->11->+1   
   */

fn byteumnoz2(s: &str) -> String {   // если старший байт справа
    let mut logic:i32 = 0;
    s.chars().map(|c| 
     if c=='0' { if logic == 0   { return '0' }               //0 -> 0
                 else { logic = 0; return '1' } }             //0+-> 1
     else {      if logic == 0 { logic =1; return '0'}        //1 ->+0
                 else          {logic = 1; return '1'} }      //1+->+1
                ).collect()
}
fn lbyteumnoz2(s: &str) -> String {   // rev если старший байт справа
    let mut logic:i32 = 0;
    s.chars().rev().map(|c| 
     if c=='0' { if logic == 0   { return '0' }               //0 -> 0
                 else { logic = 0; return '1' } }             //0+-> 1
     else {      if logic == 0 { logic =1; return '0'}        //1 ->+0
                 else          {logic = 1; return '1'} }      //1+->+1
                ).collect::<String>().chars().rev().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let stroka = &args[2].parse::<i32>().unwrap();
    println!("файл {}", file_path);
    println!("строка {}", stroka);
//    let contents = fs::read_to_string(file_path).expect("не получилось прочитать содержимое файла");     println!("полный текст файла:\n{contents}");

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut chislo:String = "-1".to_string(); 

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
            if (index+1)==(*stroka as usize) { chislo = line; }
    }
  //  assert_ne!(chislo,"-1");
    if chislo=="-1" {  panic!("не получилось найти число на указаной строке");  }
    if chislo.chars().count()!=8 { panic!("неверное количество символов"); }
//---------------------------------------------------------------------------------------//
    let copy = chislo.clone();
    println!("число до умножения {}",chislo);
    let chislo = lbyteumnoz2(&chislo);
    println!("число после умножения {} старший байт слева",chislo);
    let copy = byteumnoz2(&copy);
    println!("число после умножения {} старший байт справа",copy);
}

/*
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            // Show the line and its number.
            println!("{}. {}", index + 1, line);
        }
 */