use crate::us_tax::MaritalStatus;

pub struct TaxContext {
    pub incomes: Vec<f64>,
    pub status: MaritalStatus
}
