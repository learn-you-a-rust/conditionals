fn main() {
    // if is an expression, so it can be used
    // on the right side of a 'let' statement
    
    let condition = true;
    let number = if condition {
        5
    } else { 
        6 // note the lack of semicolon
          // on this expression
    };

    println!("The value of number is: {}", number);
}
