#![allow(unused)]

struct Point<T> {x: T, y: T,}
impl<T> Point<T> { fn xexe(&self) -> &T {&self.x} } //fn xexe(self) -> T {self.x}
impl Point<f32> { fn distancefromorigin(&self) -> f32 {(self.x.powi(2) + self.y.powi(2)).sqrt()} }

struct Point1<T, U> {x: T,y: U,}

struct Pointq<X1, Y1> {x: X1, y: Y1,}
impl<X1, Y1> Pointq<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Pointq<X2, Y2>)         ->  Pointq<X1, Y2> {
        Pointq { x: self.x, y: other.y,}                            }          //x1,y1 -> x1,y2       
}

//enum Option<T> {Some(T), None,}
//enum Result<T, E> {Ok(T), Err(E),}

fn main() {
let same = Point { x: 5, y: 1 }; //same T
println!("x = {}", same.xexe());
let same = Point { x: 5.0, y: 1.0 }; //fl,fl
println!("x = {}", same.distancefromorigin());

let sameanddif = Point1 { x: 5, y: 4.0 };

let p1 = Pointq { x: 5, y: 10.4 };
let p2 = Pointq { x: "Hello", y: 'c' };
let p3 = p1.mixup(p2);
println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
//println!("{} {}", p1.x, p1.y); moved

}