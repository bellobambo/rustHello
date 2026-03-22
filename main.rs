#![allow(warnings)]

fn main() {
    // let age : u16 = 18;
    // if age >= 18{
    //     println!("You can drive a car");
    // }else{
    //     println!("You cannot drive a car");
    // }

    // let number = 6;

    // if number % 4 == 0{
    //     println!("The number is: {}", 4);

    // }else if number % 3 == 0{
    //     println!("The number is: {}", 3);
    // }else if number % 2 == 0{
    //     println!("The number is: {}", 2);
    // }else{
    //     println!("The number is: {}", number);
    // }

    // let condition = false;
    // let number = if condition {5} else {6};

    // println!("The value of number is: {}", number);

    // loop{
    //     println!("This is an infinite loop");
    // }

    // let mut counter = 0;

    // let result = loop {

    //     counter += 1;

    //     if counter == 20{
    //         break counter - 100;
    //     };

    // };

    // println!("The value of counter is: {}", result);

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;


    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");


    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    //     break;

    // }
    // println!("LIFTOFF!!!");

    // let a = [1,2,3,4,5,6];
    // let b = ["a", "b", "c", "d", "e", "f"];

    // for letter in b {
    //     println!("The value is: {}", letter);
    // }


    let rect = (200, 500);


    struct Book {
        title: String,
        author: String,
        pages: String,
        avaliable: String,
        
    }

    struct User{
        active : bool,
        username : String,
        email : String,
        sign_in_count : u64,
    }

    let mut user1 = User{
        active : true,
        username : String::from("john_doe"),
        email : String::from("bellobambo21@gmail.com"),
        sign_in_count : 1,
    };

    // user1.email = String::from("anotheremail.com");
    println!("User 1: {}", user1.email);

    fn build_user(email : String, username : String) -> User{
        User{
            active : true,
            username : username,
            email : email,
            sign_in_count : 0, 
        }
    }

    let user2 = User{
        email : String::from("anotheremail@gmail.com"),
        ..user1

    };

    println!("User 2: {}", user2.email);


    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    struct AlwaysEqual;
    let subject = AlwaysEqual;


}
