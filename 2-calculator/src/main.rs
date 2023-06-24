use std::io;

fn write_value(input: &mut String) {
    io::stdin().read_line(input).expect("Failed to read line");
}

#[derive(Debug)]
enum Operation {
    Add(f32, f32),
    Sub(f32, f32),
    Div(f32, f32),
    Mul(f32, f32),
}

fn calculate(operation: Operation) -> f32 {
    match operation {
        Operation::Add(first_num, second_num) => first_num + second_num,
        Operation::Sub(first_num, second_num) => first_num - second_num,
        Operation::Div(first_num, second_num) => first_num / second_num,
        Operation::Mul(first_num, second_num) => first_num * second_num,
    }
}
fn main() {
    println!("------------------------");
    println!("Welcome to Calculator...");
    println!("------------------------");
    
    let mut first_number: String = String::new();
    let mut second_number: String = String::new();
    let mut operator: String = String::new();
    
    println!("Please input first number: ");
    write_value(&mut first_number);

    println!("Please choose an operator for calculating: (+, -, *, /)");
    write_value(&mut operator);

    println!("Please input second number: ");
    write_value(&mut second_number);

    let first_num: f32 = first_number.trim().parse().expect("1 - Unknown Number!");
    let second_num: f32 = second_number.trim().parse().expect("2 - Unknown Number!");
   
    let operation: Operation = match operator.trim() {
        "+" => Operation::Add(first_num, second_num),
        "-" => Operation::Sub(first_num, second_num),
        "/" => Operation::Div(first_num, second_num),
        "*" => Operation::Mul(first_num, second_num),
        _ => panic!("Unvalid Operator!!!"),
    };
    
    let result: f32 = calculate(operation);
    
    println!("Result is: {}", result);
}
