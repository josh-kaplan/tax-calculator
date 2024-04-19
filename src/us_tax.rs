use crate::tax_context::TaxContext;


#[derive(Debug)]
pub struct TaxBracketDefinition {
    pub rate: f64,
    pub lower_bound: f64,
    pub upper_bound: f64
}

impl TaxBracketDefinition {
    pub fn new(rate: f64, lower_bound: f64, upper_bound: f64) -> TaxBracketDefinition {
        TaxBracketDefinition {
            rate,
            lower_bound,
            upper_bound
        }
    }
}


pub enum MaritalStatus {
    Single,
    MarriedFilingJointly
}

pub fn get_tax_brackets(marital_status: &MaritalStatus) -> Vec<TaxBracketDefinition> {
    let s10 = TaxBracketDefinition::new(0.10, 0.0, 11000.0);
    let s12 = TaxBracketDefinition::new(0.12, 11001.0, 44725.0);
    let s22 = TaxBracketDefinition::new(0.22, 44726.0, 95375.0);
    let s24 = TaxBracketDefinition::new(0.24, 95376.0, 182100.0);
    let s32 = TaxBracketDefinition::new(0.32, 182101.0, 231250.0);
    let s35 = TaxBracketDefinition::new(0.35, 231251.0, 578125.0);
    let s37 = TaxBracketDefinition::new(0.37, 578126.0, f64::INFINITY);
    let single_tax_brackets = vec![ s10, s12, s22, s24, s32, s35, s37 ];

    let r10 = TaxBracketDefinition::new(0.10, 0.0, 22000.0);
    let r12 = TaxBracketDefinition::new(0.12, 22001.0, 89450.0);
    let r22 = TaxBracketDefinition::new(0.22, 89451.0, 190750.0);
    let r24 = TaxBracketDefinition::new(0.24, 190751.0, 364200.0);
    let r32 = TaxBracketDefinition::new(0.32, 364201.0, 462500.0);
    let r35 = TaxBracketDefinition::new(0.35, 462501.0, 693750.0);
    let r37 = TaxBracketDefinition::new(0.37, 693751.0, f64::INFINITY);
    let married_tax_brackets = vec![ r10, r12, r22, r24, r32, r35, r37 ];

    match marital_status {
        MaritalStatus::Single => single_tax_brackets,
        MaritalStatus::MarriedFilingJointly => married_tax_brackets
    }
}


pub fn compute_income_tax(ctx: &TaxContext) -> f64 {
    println!("Federal income tax breakdown:");
    let tax_brackets = get_tax_brackets(&ctx.status);
    let salary = ctx.incomes.iter().sum::<f64>();
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
    total_tax
}