fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;

    let one = x.2;
    println!("The value of  is: {one}");
    let (a1,_a2,_a3) = x;
    println!("The value of  is: {a1}");

//array
    let _a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _c = [3; 5];

}