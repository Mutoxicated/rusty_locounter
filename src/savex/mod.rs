pub trait Saveable {
    fn save(&self, buf: &mut str) {
        
    }
}

pub struct Saver {}

impl Saver {
    pub fn save_data<T>(data: Box<T>)
    where T: Saveable
    {
        if let Some(dir) = directories::BaseDirs::new() {
            let config_path = dir.config_dir();
            let path = config_path.join("Rusty_LocounterData");

            let result = std::fs::create_dir(path);
            if let Err(e) = result {
                println!("{}", e);
                return;
            }

            let mut buf = "".to_owned();

            data.save(buf.as_mut_str());
        }
    }
}