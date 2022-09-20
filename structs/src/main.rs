//#[derive(Debug)]
#![allow(unused)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user_1(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    
    let user2 = build_user_1(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };


    let user4 = User {
        email: String::from("another@example.com"),
        ..user3 //"!!!!!!!!!!!!!!!!!!!!!!!!!!1 user1 не сработает тк его использовали  юсертри и передали овнершип" 
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
 
    struct AlwaysEqual; //no store tol1ko operacii

//    struct Rectangle {
//        width: u32,
//        height: u32,
//    } //вынес вне мейна ибо его зовет функция вне мейна
   let rect1 = Rectangle {
            width: 30,
        height: 50,
    };           
    println!(
        "The area of the rectangle is {} square pixels.",
        area11(&rect1)
    );
//    println!("rect1 is {:?}", rect1);  //#[derive(Debug)]
//    dbg!(&rect1); //toje  variant

//-- metod classa sled file 
}

fn area11(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}