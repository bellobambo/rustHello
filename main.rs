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

    let weight_kg = 70.0;
    let height_m = 1.75;
    let bmi = calculate_bmi(weight_kg, height_m);
    println!("Your BMI is: {:.2}", bmi);
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

fn add(a: i32, b: i32) -> i32  {
    a + b
}


// statements

// let x = 10;
// BMI = height(kg)/height(m) * height(m)^2


fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}

