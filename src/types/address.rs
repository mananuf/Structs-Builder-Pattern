#[derive(Debug, Clone)]
pub struct Address {
    pub country: String,
    pub state: String,
    pub city: String
}

impl Default for Address{
    fn default() -> Self {
        Self {
            country: "".to_string(),
            state: "".to_string(),
            city: "".to_string(),
        }
    }
}