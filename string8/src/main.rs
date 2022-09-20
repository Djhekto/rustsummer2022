fn main() {
    let mut _s = String::new();
    
    let data = "initial contents";
    let _s = data.to_string();

    let _s = "initial contents".to_string();
    let _hello = String::from("Здравствуйте"); //ymolchanie

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}",s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); //norm

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //takes ownership of s1, appends a copy of the contents of s2

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = format!("{}-{}-{}", s1, s2, s3);
    //"!!
    let hello = "Здравствуйте";
    let _s = &hello[0..4];
    println!("{}",_s);
    //"!!
    for c in "नमस्ते".chars() {
        println!("{}", c);
    } //norm
}
