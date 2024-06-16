mod pst;
mod file;

use std::{collections::HashMap, fs::{self, ReadDir}, io::Read};
use file::EFile;

pub struct Results {
    pub loc:usize,
    pub files:Vec<EFile>
}

impl Results {
    pub fn new() -> Self {
        Self {
            loc:0,
            files: Vec::new()
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
    common_extensions: Vec<String>,

    pub results: Option<Result<Results, AppError>>
}

macro_rules! to_owned_vec {
    ($($e:expr),*) => {
        vec![
            $($e.to_owned()),*
        ]
    };
}

impl App {
    pub fn new(cdir:&str) -> Self {
        Self {
            path_to_check: None,
            file_extensions: None,
            folders_to_ignore: None,
            current_path: cdir.to_owned(),
            common_extensions: to_owned_vec![
                "rs", "go",
                "lua", "cs",
                "js", "ts",
                "py", "java",
                "php", "rb",
                "asm", "pl",
                "mat", "htm",
                "html", "sh",
                "dart", "swift",
                "vb", "c",
                "cpp"
            ],
            results: None,
        }
    }

    pub fn set_current_path(&mut self, path:&str) {
        self.current_path = path.to_owned();
    }

    pub fn get_current_path(&self) -> &str {
        self.current_path.as_str()
    }

    pub fn set_path(&mut self, ptc:&str) {
        self.path_to_check = Some(ptc.to_owned());

        let entries = fs::read_dir(
            self.path_to_check
            .as_ref()
            .unwrap()
            .as_str()
        );

        let mut exts:HashMap<String, usize> = HashMap::new();

        self.get_common_extensions(entries, &mut exts);

        self.folders_to_ignore = Some(Vec::new());
        self.file_extensions = Some(Vec::new());

        for ext in &exts {
            if *ext.1 < 2 {
                continue;
            }

            if ext.0 == "rs" {
                self.add_folder("target");
            }
            if ext.0 == "c" {
                self.add_folder("lib");
                self.add_folder("include");
            }

            self.add_extension(ext.0);
        }
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
    
    pub fn remove_extension(&mut self, i: usize) {
        if self.file_extensions.is_none() {
            return;
        }
        let extensions = self.file_extensions.as_mut().unwrap();
        extensions.remove(i);
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

    pub fn remove_folder(&mut self, i: usize) {
        if self.folders_to_ignore.is_none() {
            return;
        }
        let folders = self.folders_to_ignore.as_mut().unwrap();
        folders.remove(i);
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

    fn get_common_extensions(&self, entries: Result<ReadDir, std::io::Error>, exts:&mut HashMap<String, usize>) {
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
                self.get_common_extensions(fs::read_dir(entrypath), exts);
            }else if meta.is_file() {
                let file = fs::File::open(&entrypath);
                if file.is_err() {
                    continue
                }
                let ext_os = entrypath.extension();
                if ext_os.is_none() {
                    continue
                }
                let ext = ext_os.unwrap().to_str();
                if ext.is_none() {
                    continue
                }
                let ext = ext.unwrap().to_owned();

                if !self.extension_is_common(ext.as_str()) {
                    continue
                }
                exts.entry(ext.clone()).or_insert(1);
                exts.entry(ext).and_modify(|v| { *v += 1 });
            }   
        }
    }

    fn extension_is_common(&self, ext:&str) -> bool {
        for bext in &self.common_extensions {
            if bext == ext {
                return true
            }
        }
        false
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
                if !self.extension_is_valid(&entrypath) {
                    continue
                }
                let file = fs::File::open(entrypath);
                if file.is_err() {
                    continue
                }
                let mut file = file.unwrap();
                let mut buf: Vec<u8> = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                let loc = get_loc(&buf);
                res.loc += loc;
                let folder = EFile::new(entry.file_name().to_str().unwrap(), loc);
                res.files.push(folder);
                res.files.sort();
            }   
        }
    }

    fn extension_is_valid(&self, entrypath: &std::path::Path) -> bool {
        if self.file_extensions.is_none() {
            return true
        }
    
        for ext in self.file_extensions.as_ref().unwrap() {
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
        return false
    }

    for end in ends.as_ref().unwrap() {
        if entrypath.ends_with(end) {
            return true
        }
    }

    false
}


fn get_loc(buf:&Vec<u8>) -> usize {
    let mut loc:usize = 1;
    for byte in buf {
        if *byte == b'\n' {
            loc += 1;
        }
    }
    loc
}
