use std::path::PathBuf;

pub struct Line {
    pub content: String,
    pub location: usize,
    pub size:usize,
    pub path:Option<String>,
}

impl std::cmp::Eq for Line {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.size == other.size && self.location == other.location
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        if self.size < other.size {
            return Less
        }
        if self.size == other.size {
            return Equal
        }
        Greater
    }
}