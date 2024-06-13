
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
    folders_to_ignore: Option<Vec<String>>,
    current_path: String,

    pub results: Option<Result<Results, AppError>>
}

impl App {
    pub fn new(cdir:&str) -> Self {
        Self {
            path_to_check: None,
            file_extensions: None,
            folders_to_ignore: None,
            current_path: cdir.to_owned(),
            results: None,
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

    pub fn add_extension(&mut self, ext_name:&str) {
        if self.file_extensions.is_none() {
            self.file_extensions = Some(Vec::new());
        }
        let extensions = self.file_extensions.as_mut().unwrap();
        extensions.push(ext_name.to_owned());
    }

    pub fn get_extension(&mut self, i:usize) -> &mut String {
        let exts = self.file_extensions.as_mut().unwrap();

        &mut exts[i]
    }

    pub fn iterate_extensions(&mut self) -> Option<core::slice::Iter<String>> {
        self.file_extensions.as_ref()?;

        let extensions = self.file_extensions.as_mut().unwrap();

        Some(extensions.iter())
    }

    pub fn add_folder(&mut self, ext_name:&str) {
        if self.folders_to_ignore.is_none() {
            self.folders_to_ignore = Some(Vec::new());
        }
        let folders = self.folders_to_ignore.as_mut().unwrap();
        folders.push(ext_name.to_owned());
    }

    pub fn get_folder(&mut self, i:usize) -> &mut String {
        let exts = self.folders_to_ignore.as_mut().unwrap();

        &mut exts[i]
    }

    pub fn iterate_folders(&mut self) -> Option<core::slice::Iter<String>> {
        self.folders_to_ignore.as_ref()?;

        let folders = self.folders_to_ignore.as_mut().unwrap();

        Some(folders.iter())
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
            if meta.is_err() {
                continue
            }

            let meta = meta.unwrap();
            let entrypath = entry.path();
            if meta.is_dir() {
                if is_hidden_folder(entrypath.as_path()) {
                    continue
                }
                if has_ending(&entrypath, &self.folders_to_ignore) {
                    continue
                }
                self.dig_entries(res, fs::read_dir(entrypath));
            }else if meta.is_file() {
                if !extension_is_valid(&entrypath, &self.file_extensions) {
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

fn is_hidden_folder(entrypath: &std::path::Path) -> bool {
    if entrypath.ends_with(".git") {
        return true
    }
    if entrypath.ends_with(".vscode") {
        return true
    }

    false
}

fn has_ending(entrypath: &std::path::Path, ends:&Option<Vec<String>>) -> bool {
    if ends.is_none() {
        return true
    }

    for end in ends.as_ref().unwrap() {
        if entrypath.ends_with(end) {
            return true
        }
    }

    false
}

fn extension_is_valid(entrypath: &std::path::Path, exts:&Option<Vec<String>>) -> bool {
    if exts.is_none() {
        return true
    }

    for ext in exts.as_ref().unwrap() {
        let extension = entrypath.extension();
        if extension.is_none() {
            return false
        }
        let extension = extension.unwrap();
        if extension.to_str().unwrap() == ext {
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
