use std::{fs::File, io::Write};

pub trait Saveable {
    fn save(&self) -> Vec<u8>;
}

impl<T: Saveable> Saveable for Vec<T> {
    fn save(&self) -> Vec<u8>
    {
        let slice = unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of_val(self))
        };

        slice.to_owned()
    }
}

impl Saveable for String {
    fn save(&self) -> Vec<u8>
    {
        let slice = unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, std::mem::size_of_val(self))
        };

        slice.to_owned()
    }
}

#[allow(non_camel_case_types)]
pub struct er {}

impl er {
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

            let bytes = data.save();

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

            println!("Bytes {:?}", bytes);

            let result = file.write_all(&bytes);
            if let Err(e) = result {
                println!("Failed to write file: {}", e);
            }
        }
    }
}