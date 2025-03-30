fn main() {
    // TODO: Change the line below to fix the compiler error.
    //let x; 

    //if x == 10 {
    //    println!("x is ten!");
    //} else {
    //    println!("x is not ten!");
    //}

    // Rust does not allow uninitialized variables, so:
    let x = 10; 
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }

    // Optimize code
    let x = 10; 
    println!("{}", if x == 10 {"x is ten!"} else {"x is not ten!"}); 
    
    // Another option 
    let x = 10; 
    match x {
        10 => println!("x is ten!"),
        _ => println!("x is not ten!"),
    }
}
