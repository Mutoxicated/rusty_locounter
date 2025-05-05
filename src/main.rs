mod app;
mod ui;

use iced::{widget::Column, Settings};

struct Program {
    app:app::App
}

impl Default for Program {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let stred = current_dir.to_str().unwrap();
        let app = app::App::new(stred);

        Self {
            app
        }
    }
}



impl Program {
    pub fn view(&self) -> Column<i32> {
        return Column::new()
    }

    pub fn update(&mut self, message: i32) {

    }

}

pub fn main() -> iced::Result {
    iced::run("LOC Counter", Program::update, Program::view)
}

