fn main() {
    println!("Hello, 🦀🦀 from cargo!");
    let x = 5;

    let x = x + 1;
    let x = x * 1000;

    {
        let x = x * 2;
        println!("The Value of x in the inner scope : {x}");
    }

    println!("Normal Valur of x : {}", x)
}

const PI: f64 = 3.14159;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
