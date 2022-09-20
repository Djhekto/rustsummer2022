fn main() {
    let x: i32 = 5;
//shad
    let x = x + 1;

    { //private shad
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
//const cant be set to mut
/*
    let spaces = "   ";
    let spaces = spaces.len();  shad !

    let mut spaces = "   ";
    spaces = spaces.len();        mut no!

*/
//  let y: f32 = 3.0; // f32
//let x = 2.0; // f64
