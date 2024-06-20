use std::{fs::File, io::Read, mem::transmute};

#[allow(non_camel_case_types)]
pub struct tracter {}

impl tracter {
    pub fn extract<T: Extractable>() -> Option<T> {
        if let Some(dir) = directories::BaseDirs::new() {
            let config_path = dir.config_dir();
            let path = config_path.join("Rusty_LocounterData");

            let result = std::fs::read_dir(&path);

            if result.is_err() {
                let result = std::fs::create_dir(&path);
                if let Err(e) = result {
                    println!("{}", e);
                    return None
                }
            }

            let abs_path = path.canonicalize().expect("Failed to canonicalize path");
            assert!(abs_path.exists());


            let result = File::options()
                .read(true)
                .write(false)
                .open(abs_path.join("data.savex"));
            if let Err(e) = result {
                println!("Failed to create file: {}", e);
                return None
            }

            let mut buf:Vec<u8> = Vec::new();
            let res = result.unwrap().read_to_end(&mut buf);
            if let Err(e) = res {
                println!("Failed to read file: {}", e);
                return None
            }

            return Some(T::extract(buf))
        }

        None
    }

    // fn consume(&mut self) -> String {
    //     let mut temp:String = String::new();
    //     //buf[self.current] != buf.len()-1
    //     if self.current == self.buf.len()-1 {
    //         let str = self.buf[self.current].to_string();
    //         self.current += 1;
    //         return str
    //     }
    //     if self.buf[self.current].is_alphanumeric() {
    //         while self.buf[self.current].is_alphanumeric() {
    //             temp.push(self.buf[self.current]);
    //             self.current += 1;
    //         }
    //         temp
    //     } else if self.buf[self.current].is_alphabetic(){
    //         while self.buf[self.current].is_alphabetic() {
    //             temp.push(self.buf[self.current]);
    //             self.current += 1;
    //         }
    //         temp
    //     } else{
    //         let str = self.buf[self.current].to_string();
    //         self.current += 1;
    //         str
    //     }
    // }
}

impl<T: Extractable> Extractable for Vec<T> {
    fn extract(bytes: Vec<u8>) -> Self {
        unsafe {
            let val:Self = transmute(bytes);
            val
        }
    }
}

impl Extractable for String {
    fn extract(bytes: Vec<u8>) -> Self {
        unsafe {
            let val:Self = transmute(bytes);
            val
        }
    }
}

pub trait Extractable {
    fn extract(bytes: Vec<u8>) -> Self;
}