use std::{fmt::Display, fs::File, io::Write};

pub trait Saveable {
    fn save(&self) -> String;
}

impl<T: Saveable> Saveable for Vec<T> {
    fn save(&self) -> String
    where T: Saveable
    {
        let mut string = String::new();
        string.push_str(write_code(SaveCode::Vec).as_str());
        string.push('[');
        for t in self {
            string.push_str(t.save().as_str());
        }
        string.push(']');
        string
    }
}

impl Saveable for String {
    fn save(&self) -> String
    {
        write_code(SaveCode::String)+self.as_str()
    }
}

fn write_code(savecode:SaveCode) -> String {
    savecode.to_string()
}

#[derive(Clone, Copy)]
enum SaveCode {
    Vec,
    String,
}


impl Display for SaveCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = *self as usize;
        if num == 0 {
            return f.write_str("0000")
        }

        let mut a = (num as f32)/1000.0;
        
        let numstr = num.to_string();
        let mut finished = "".to_owned();

        while a < 1.0 {
            finished.push('0');
            a *= 10.0
        }
        finished.push_str(numstr.as_str());

        f.write_str(finished.as_str())
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