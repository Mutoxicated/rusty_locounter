mod savecodes;
mod extracter;
mod node;

use savecodes::{SaveCode, write_code};
use std::{fs::File, io::Write};

use crate::code;

pub trait Saveable {
    fn save(&self) -> String;
}

pub trait Extractable {
}

impl<T: Saveable> Saveable for Vec<T> {
    fn save(&self) -> String
    {
        let mut string = String::new();
        string.push_str(code!(Vec).as_str());
        string.push_str(code!(WrapStart).as_str());
        for t in self {
            string.push_str(t.save().as_str());
        }
        string.push_str(code!(WrapEnd).as_str());
        string
    }
}


impl Saveable for String {
    fn save(&self) -> String
    {
        code!(StringStart)+self.as_str()+code!(StringEnd).as_str()
    }
}

pub struct Saver {}

impl Saver {
    pub fn save_data<T>(data: T)
    where T: Saveable
    {
        if let Some(dir) = directories::BaseDirs::new() {
            let config_path = dir.config_dir();
            let path = config_path.join("Rusty_LocounterData");

            let result = std::fs::read_dir(&path);

            if result.is_err() {
                let result = std::fs::create_dir(&path);
                if let Err(e) = result {
                    println!("{}", e);
                    return;
                }
            }

            let abs_path = path.canonicalize().expect("Failed to canonicalize path");
            assert!(abs_path.exists());

            let string = data.save();

            let result = File::options()
                .read(false)
                .write(true)
                .truncate(true)
                .create(true)
                .open(abs_path.join("data.savex"));
            if let Err(e) = result {
                println!("Failed to create file: {}", e);
                return;
            }

            let mut file = result.unwrap();

            let result = file.write_all(string.as_bytes());
            if let Err(e) = result {
                println!("Failed to write file: {}", e);
            }
        }
    }
}