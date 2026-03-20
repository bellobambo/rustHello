fn main() {
    hello_world();
    tell_height(20);
    human_id("John Doe", 30, 175.5);

    let x = {
        let price  = 5;
        let quantity  = 10;
        price * quantity
    };

    println!("Result is: {}", x);
    let sum = add(4, 6);
    println!("Sum is: {}", sum);
    println!("value from function 'add' is: {}", add(4,6));
}

fn hello_world() {
    println!("Hello, World! 🦀🦀");
}

fn tell_height(height: u32) {
    println!("Your height is {} cm", height);
}


fn human_id(name : &str, age: u32, height : f32){
    println!("My Name is: {}, i am : {} years old, and my height is: {} cm", name, age, height);
}



// expression

fn add(a: i32, b: i32) -> i32 {
    a + b
}
