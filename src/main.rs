use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::str::FromStr;
use crate::us_tax::MaritalStatus;
use crate::tax_context::TaxContext;

mod us_tax;
mod tax_context;


fn main() {
    // Get necessary info from user
    let mstatus = prompt::<i32>("Status (1 - Single, 2 - Married filing jointly): ");
    let mstatus = if mstatus == 1 { 
        MaritalStatus::Single 
    } else {
        MaritalStatus::MarriedFilingJointly 
    };
    let salary = prompt::<f64>("Enter your total income: ");
    println!("");

    let tax_ctx = TaxContext {
        incomes: vec![salary],
        status: mstatus
    };

    // Get tax brackets
    //let tax_brackets = us_tax::get_tax_brackets(mstatus);

    // Compute federal income tax
    let federal_income_tax = us_tax::compute_income_tax(&tax_ctx);
    // Compute medicare and social security
    let medicare = 0.0145 * salary;
    let social_security = 0.062 * salary;

    println!("");
    println!("Federal income tax: ${:10.2}", federal_income_tax);
    println!("Medicare:           ${:10.2}", medicare);
    println!("Social security:    ${:10.2}", social_security);
    println!("");
    println!("Total tax is ${:.2}", federal_income_tax + medicare + social_security);
}

/*
 * Prompts the user with a string message and converts their response to a specified numeric type T. 
 */
fn prompt<T>(msg: &str) -> T where T: FromStr, <T as FromStr>::Err: Debug {
    // Prompt user
    print!("{}", msg);
    io::stdout().flush().unwrap();

    // Initialize input string buffer and read user input
    let mut salary = String::new();
    
    io::stdin()
        .read_line(&mut salary)
        .expect("Failed to read line");

    // Parse as float adn return
    let salary = numeric_converter(&salary);
    salary
}

/**
 * Performs the generic numeric conversion needed for the prompt function.
 * Takes in a string and converts it to the specified numeric type T.
 */
fn numeric_converter<T>(s: &str) -> T where T: FromStr, <T as FromStr>::Err: Debug {
    let result: T = s.trim().parse().unwrap();
    result
}


#[test]
fn test_generic_numeric_converter() {
    assert_eq!(numeric_converter::<f64>("1000.0"), 1000.0);
    assert_eq!(numeric_converter::<i32>("21"), 21);
    assert_eq!(numeric_converter::<u32>("21"), 21);
}


