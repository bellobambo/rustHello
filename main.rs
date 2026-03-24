fn main() {
    // let _v: Vec<i32> = Vec::new();
    // let mut _vec: Vec<i32> = Vec::new();
    // let mut _vec: Vec<i32> = vec![1, 2, 3];

    // _vec.push(5);
    // _vec.push(6);
    // _vec.push(7);
    // _vec.push(8);
    // _vec.push(9);

    // println!("The numbers vector is: {:?}", _vec);

    // let _v = vec![1, 2, 3, 4, 5];

    // // let third  : &i32 = &_v[2];
    // // println!("The third element is: {}", third);

    // let third = _v.get(2);
    // match third {
    //     Some(third) => println!("The third element is: {}", third),
    //     None => println!("There is no third element."),
    // }


    let s = "whatever".to_string();
    let s = String::from("whatever");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 
    let mut s = String::from("foo");

    s.push_str("bar");
    s.push('!');
    // println!("The string is: {}", s);
    println!("The string is: {}", s3);


}
