use std::io::stdin;
use std::vec::Vec;

fn main() {
    let mut input : String = String::new();
    stdin().read_line(&mut input).expect("Error entering value");
    
    let mut input: i64 = input.trim().parse().expect("You didn't write a number");

    let mut num1 = 0;
    let mut num2 = 1;

    let mut series: Vec<i64> = Vec::new();

    loop {
        if input <= 0 {
            break
        }
        input -= 2;
        series.push(num1);      
        series.push(num2);
        
        num1 = num1 + num2;
        num2 = num1 + num2;  
    }

    println!("The Fibonacci series of {} is {:?}", input, series);
}
