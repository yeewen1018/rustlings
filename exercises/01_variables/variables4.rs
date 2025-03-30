// TODO: Fix the compiler error.
fn main() {
    //let x = 3;
    //println!("Number {x}");

    //x = 5; // Don't change this line
    //println!("Number {x}");

    // Rust variables are immutable by default, 
    // so if we attempt the above code, where we try to 
    // make a change to the x variable, then the compiler
    // will throw an error, so: 

    let mut x = 3; 
    println!("Number {x}"); 

    x = 5; 
    println!("Number {x}"); 
}
