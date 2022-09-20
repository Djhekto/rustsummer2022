fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    
    println!("{}",v[4]);//5

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {  //get  no panic
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v { //mut -> cmozem menyat1 vnytr1 cikla
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![ //vec s neodinak elem*(const) //ne const ->17 glava  a et tok 8
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
/*fn main() {
    let v: Vec<i32> = Vec::new();
}
//
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];  meshaet

    v.push(6);

    println!("The first element is: {}", first);
*/