// TODO: Fix the function body without changing the signature.
// Solution: Remove the semicolon from num * num to explicitly return the result. 
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
