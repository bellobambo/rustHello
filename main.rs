fn divideResult (numerator : f64, denominator : f64)-> Result<f64, String>{
    if denominator == 0.0 {
        Err("Error: Division by zero".into())
    }else{
        Ok(numerator/denominator)
    }
}

fn main (){
    

match divideResult(100.23, 00.0){
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Issues Day: {}", e),
}

//    enum OPTION<T>{
//     Some(T),
//     None,
//    }

//    enum Result<T,E>{
//     Ok(T),
//     Err(E), 
//    }

//    fn divide (numerator : f64, demoninator : f64)-> OPTION<f64>{
//     if demoninator == 0.0 {
//         OPTION::None
//     }else{
//         OPTION::Some(numerator/demoninator)
//     }
//    }



    println!("Hello, world!");
}


fn divideOption (numerator : f64, demoninator : f64)-> Option<f64>{
    if demoninator == 0.0 {
        None
    }else{
        Some(numerator/demoninator)
    }
}