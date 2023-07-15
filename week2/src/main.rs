use std::io::{self, Write};


// Operation enum class
enum Operation {
    Add(f64,f64),
    Substract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64)
}

// calculate function for operating equation and calculate
fn calculate (operation: Operation) -> Result<f64, &'static str> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Substract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b != 0.0 { // checks if divisor not equal to zero
                Ok(a / b)
            } else {
                Err("Divide by zero error.")
            }
        }
    }
}

// store user input to buffer that provided
fn get_input(buff:&mut String, prompt:&str) {
    println!("{}", prompt); // print given prompt text
    io::stdout().flush().unwrap(); // clear remain output if exists
    buff.clear(); // clear buffer
    std::io::stdin().read_line(buff).unwrap(); // insert text to buffer
}

fn main() {

    // input buffer
    let mut _input = String::new();
    
    // get first number parameter 
    get_input(&mut _input, "Enter first number: ");
    let first_number: f64 = _input.trim().parse().unwrap();

    // get operation parameter
    get_input(&mut _input, "Enter operation (+,-,*,/): ");
    let operation_symbol: char = _input.trim().chars().next().unwrap();

    // get second number parameter
    get_input(&mut _input, "Enter second number: ");
    let second_number: f64 = _input.trim().parse().unwrap();

    // operation enum instance
    let operation = match operation_symbol {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Substract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid Operation"),
    };

    // it prints the result or if any complications occur, it prints error.
    match calculate(operation) {
        Ok(result) => println!("Result: {}{}{}={}", first_number, operation_symbol, second_number, result),
        Err(error) => println!("Error: {}", error)
    }

}

