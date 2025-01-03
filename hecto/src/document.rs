use crate::Position;
use crate::Row;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    file_name: Option<String>,
    dirty: bool,
}

impl Document {
    /// # Errors
    /// pass on errors from reading the filesystems.
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self {
            rows,
            file_name: Some(filename.to_owned()),
            dirty: false,
        })
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    // TODO: can this search be incrementalized? so we can do incremental search?
    pub fn find(&self, query: &str) -> Option<Position> {
        for (y, row) in self.rows.iter().enumerate() {
            if let Some(x) = row.find(query) {
                return Some(Position { x, y });
            }
        }
        None
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[must_use]
    pub fn file_name(&self) -> &Option<String> {
        &self.file_name
    }

    pub fn set_file_name(&mut self, new_name: Option<String>) {
        self.file_name = new_name;
    }

    /// # Errors
    /// pass on errors from creating a file handle and writing to it.
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    fn insert_newline(&mut self, at: &Position) {
        if at.y > self.len() {
            return;
        }
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        #[allow(clippy::indexing_slicing)]
        let new_row = self.rows[at.y].split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn insert(&mut self, at: &Position, ch: char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if ch == '\n' {
            self.insert_newline(at);
            return;
        }
        if at.y == self.len() {
            // new row
            let mut row = Row::default();
            row.insert(0, ch);
            self.rows.push(row);
        } else {
            // existing row
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, ch);
        }
    }

    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        #[allow(clippy::indexing_slicing)]
        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
    }
}
