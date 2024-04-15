use std::io;
use std::io::Write;

#[derive(Debug)]
struct TaxBracketDefinition {
    rate: f64,
    lower_bound: f64,
    upper_bound: f64
}

impl TaxBracketDefinition {
    fn new(rate: f64, lower_bound: f64, upper_bound: f64) -> TaxBracketDefinition {
        TaxBracketDefinition {
            rate,
            lower_bound,
            upper_bound
        }
    }
}


fn main() {
    // Get salaray as f64 from user
    print!("Enter your salary: ");
    io::stdout().flush().unwrap();

    let mut salary = String::new();

    io::stdin()
        .read_line(&mut salary)
        .expect("Failed to read line");

    let salary: f64 = salary.trim().parse().expect("Please type a number!");

    let r10 = TaxBracketDefinition::new(0.10, 0.0, 22000.0);
    let r12 = TaxBracketDefinition::new(0.12, 22001.0, 89450.0);
    let r22 = TaxBracketDefinition::new(0.22, 89451.0, 190750.0);
    let r24 = TaxBracketDefinition::new(0.24, 190751.0, 364200.0);
    let r32 = TaxBracketDefinition::new(0.32, 364201.0, 462500.0);
    let r35 = TaxBracketDefinition::new(0.35, 462501.0, 693750.0);
    let r37 = TaxBracketDefinition::new(0.37, 693751.0, f64::INFINITY);

    let tax_brackets = vec![
        r10,
        r12,
        r22,
        r24,
        r32,
        r35,
        r37,
    ];

    println!("");

    // Compute federal income tax
    println!("Federal income tax breakdown:");
    let mut remaining = salary;
    let mut total_tax = 0.0;
    for bracket in tax_brackets.iter() {
        // println!("{:?}", bracket);
        if salary > bracket.upper_bound {
            let delta = bracket.upper_bound - bracket.lower_bound;
            let tax = bracket.rate * delta;
            total_tax += tax;
            remaining = remaining - (bracket.upper_bound - bracket.lower_bound);
            println!("  Tax @ {:.1}% rate is ${:.2} (based on ${:.2}. ${:.2} remaining)", 
                bracket.rate * 100.00, tax, delta, remaining);
            
        } else {
            let tax = bracket.rate * remaining;
            total_tax += tax;
            println!("  Tax @ {:.1}% rate is ${:.2} (based on ${:.2}. ${:.2} remaining)", 
                bracket.rate * 100.00, tax, remaining, 0.00);
            break;
        }
    }

    // Compute medicare and social security
    let medicare = 0.0145 * salary;
    let social_security = 0.062 * salary;

    println!("");
    println!("Federal income tax: ${:10.2}", total_tax);
    println!("Medicare:           ${:10.2}", medicare);
    println!("Social security:    ${:10.2}", social_security);
    println!("");
    println!("Total tax is ${:.2}", total_tax + medicare + social_security);
}
