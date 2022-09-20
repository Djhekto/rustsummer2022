#![allow(unused)]

fn main() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = //_ tk E razn patterns
        teams.into_iter().zip(initial_scores.into_iter()).collect();//7777
        //into iter  -> +var 4 iter
        //zip ...->(..,..)
        //collect  anti into_iter
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{}",field_name); map zabral ownership
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); //10->25  zamena
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}",score); //po imeni nashlo chislo
    let team_name = String::from("ahahahahah");
    let score = scores.get(&team_name);
    println!("{:?}",score); //bez imeni nashlo none

    scores.entry(String::from("hehe")).or_insert(68); //zapusti progu i poime1sh1

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //chitaet kvo slova
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
