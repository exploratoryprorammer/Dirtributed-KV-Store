pub struct Storage {
    pub current_term: u64,
    pub voted_for: Option<usize>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            current_term: 0,
            voted_for: None,
        }
    }
}