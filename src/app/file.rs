pub struct EFile {
    pub name: String,
    pub loc: usize
}

impl std::cmp::Eq for EFile {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for EFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.loc == other.loc
    }
}

impl PartialOrd for EFile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EFile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        if self.loc < other.loc {
            return Less
        }
        if self.loc == other.loc {
            return Equal
        }
        Greater
    }
}

impl EFile {
    pub fn new(name: &str, loc: usize) -> Self {
        Self {
            name:name.to_owned(),
            loc
        }
    }
}