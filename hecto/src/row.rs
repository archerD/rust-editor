use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
            len: value.graphemes(true).count(),
        }
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(end, start);
        // self.string.get(start..end).unwrap_or_default().to_string()
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                // TODO: better handling of tabs (i.e., 4 spaces, or configurable spaces per tab)
                result.push_str(&" ".repeat(4));
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn insert(&mut self, at: usize, ch: char) {
        if at > self.len() {
            self.string.push(ch);
            self.len += 1;
        } else {
            let mut result: String = String::new();
            let mut length = 0;
            for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
                if index == at {
                    result.push(ch);
                    length += 1;
                }
                result.push_str(&grapheme);
                length += 1;
            }
            self.string = result;
            self.len = length;
        }
    }

    #[must_use]
    pub fn split(&mut self, at: usize) -> Self {
        let mut fst_str: String = String::new();
        let mut snd_str: String = String::new();
        let mut fst_len = 0;
        let mut snd_len = 0;

        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                fst_str.push_str(grapheme);
                fst_len += 1;
            } else {
                snd_str.push_str(grapheme);
                snd_len += 1;
            }
        }

        self.string = fst_str;
        self.len = fst_len;
        Self { string: snd_str, len: snd_len, }
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                result.push_str(grapheme);
                length += 1;
            }
        }
        self.string = result;
        self.len = length;
    }
}
