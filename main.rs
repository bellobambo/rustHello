


fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array : {:?}", numbers);

    // let mix = [1,2, "apple", true];
    // println!("Mix Array : {:?}", mix)

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    // Tuples

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human char : {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mixed tuple : {:?}", my_mix_tuple);

    // Slices
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slices : {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slices : {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"IT".to_string(),
        &"Harry Porter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("Book Slices : {:?}", book_slices);

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value : {}", slice);
}

fn print() {
    println!("SLICE: {}", slice)
}



fn hello_world(){
    println!("Hello, World! 🦀🦀");
}