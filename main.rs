fn main (){
    let numbers : [i32; 5] = [1,2,3,4,5];
    println!("Number Array : {:?}", numbers);

    // let mix = [1,2, "apple", true];
    // println!("Mix Array : {:?}", mix)

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);


    // Tuples

    let human : (String, i32, bool) = ("Alice", 30, false);
    println!("Human char : {:?}", human)
}