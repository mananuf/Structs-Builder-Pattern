#[derive(Debug, Clone)]
pub enum AccountType {
    Savings,
    Current
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Savings
    }
}