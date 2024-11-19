use std::cmp;

pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self { string: String::from(value), }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(end, start);
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}

