// LOCOUNTER
#![feature(associated_type_bounds)]

mod app;
mod dresser;
mod savex;

use dresser::Dresser;
use savex::sav;
use savex::ex;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([700.0,700.0])
            .with_resizable(false)
            .with_minimize_button(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    let data = "test".to_owned();
    sav::er::save_data(data);

    let extracter = ex::tracter::extract::<String>().unwrap();

    eframe::run_native(
        "LOC Counter", 
        options, 
        Box::new(|_| {
            let current_dir = std::env::current_dir().unwrap();
            let stred = current_dir.to_str().unwrap();
            
            Box::<Dresser>::new(Dresser::new(stred))
        })
    ).unwrap();
}
