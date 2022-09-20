#![allow(unused)]

mod back_of_house 
{
pub struct Breakfast { pub toast: String, seasonal_fruit: String,}   //по умолчанию приват
impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
           return Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),}
                                                }
                }

#[derive(Debug)] pub enum Appetizer { Soup, Salad,}       //по умолчанию паблик //derive dlya order1
            
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("I'd like {:?} as appetazer", order1);    
}