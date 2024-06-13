
use std::{fmt::Display, fs::{self, ReadDir}, io::Read};

pub struct Results {
    pub loc:usize,
    pub files:Vec<String>
}

impl Results {
    pub fn new() -> Self {
        return Self {
            loc:0,
            files: Vec::new()
        }
    }

    pub fn combine(&mut self, other:Results) {
        self.loc += other.loc;
        for file in &other.files {
            self.files.push(file.clone())
        }
    }
}

pub enum AppError {
    NoPathToCheck
}

impl AppError {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NoPathToCheck => {
                "You didn't set a path to your project folder!"
            }
        }
    }
}

pub struct App {
    path_to_check: Option<String>,
    file_extensions: Option<Vec<String>>,
    current_path: String,
    pub results: Option<Result<Results, AppError>>
}

impl App {
    pub fn new(cdir:&str) -> Self {
        Self {
            path_to_check: None,
            file_extensions: None,
            current_path: cdir.to_owned(),
            results: None
        }
    }

    pub fn get_current_path(&self) -> &str {
        self.current_path.as_str()
    }

    pub fn set_path(&mut self, ptc:&str) {
        self.path_to_check = Some(ptc.to_owned())
    }

    pub fn get_path(&self) -> Option<&str> {
        if let Some(str) = &self.path_to_check {
            Some(str.as_str())
        }else {
            None
        }
    }

    pub fn action(&mut self) {
        if self.path_to_check.is_none() {
            self.results = Some(Err(AppError::NoPathToCheck));
            return;
        }
        let entries = fs::read_dir(
            self.path_to_check
            .as_ref()
            .unwrap()
            .as_str()
        );
        
        let mut results = Results::new();

        self.dig_entries(&mut results, entries);

        self.results = Some(Ok(results))
    }

    fn dig_entries(&self, res:&mut Results, entries: Result<ReadDir, std::io::Error>) {
        if entries.is_err() {
            return;
        }
    
        for entry in entries.unwrap() {
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            let meta = entry.metadata();
            if let Ok(x) = meta {
                if x.is_dir() {
                    if entry.path().to_str().unwrap().contains('.') {
                        continue
                    }
                    self.dig_entries(res, fs::read_dir(entry.path()));
                }else if x.is_file() {
                    let entrypath = entry.path();
                    if !supports_extensions(&entrypath, &self.file_extensions) {
                        continue
                    }
                    let file = fs::File::open(entrypath);
                    if file.is_err() {
                        continue
                    }
                    let mut file = file.unwrap();
                    let mut buf: Vec<u8> = Vec::new();
                    file.read_to_end(&mut buf).unwrap();
                    res.loc += get_loc(&buf);
                    res.files.push(entry.file_name().into_string().unwrap())
                }
            }
        }
    }
}



fn supports_extensions(entry_path: &std::path::PathBuf, exts:&Option<Vec<String>>) -> bool {
    if exts.is_none() {
        return true
    }

    for ext in exts.as_ref().unwrap() {
        if entry_path.ends_with(ext) {
            return true
        }
    }

    false
}

fn get_loc(buf:&Vec<u8>) -> usize {
    let mut loc:usize = 0;
    for byte in buf {
        if *byte == b'\n' {
            loc += 1;
        }
    }
    loc
}
