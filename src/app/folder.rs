use std::borrow::BorrowMut;

use crate::app::file::EFile;

pub struct EFolder {
    name: String,
    next: Vec<EFolder>,
    files: Option<Vec<EFile>>,
    loc: usize
}

impl std::cmp::Eq for EFolder {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for EFolder {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.loc == other.loc
    }
}

impl PartialOrd for EFolder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EFolder {
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

impl EFolder {
    pub fn root(name:String) -> Self {
        Self {
            name,
            next:Vec::new(),
            files:None,
            loc:0,
        }
    }

    pub fn add_next(&mut self, next:EFolder) {
        self.next.push(next);
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn loc(&self) -> usize {
        self.loc
    }

    pub fn next(&mut self) -> &mut Vec<EFolder> {
        &mut self.next
    }

    pub fn next_immut(&self) -> &Vec<EFolder> {
        &self.next
    }

    pub fn latest_next(&mut self) -> &mut EFolder {
        let len = self.next.len();
        self.next[len-1].borrow_mut()
    }
 
    pub fn add_file(&mut self, efile:EFile) {
        if self.files.is_none() {
            self.files = Some(Vec::new())
        }

        self.loc += efile.loc;

        self.files.as_mut().unwrap().push(efile);
    }

    pub fn get_files(&self) -> Option<&Vec<EFile>> {
        self.files.as_ref()
    }

    pub fn sort(&mut self) {
        self.files.as_mut().unwrap().sort();
    }
}