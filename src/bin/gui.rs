use iced::button;
use iced::{ Settings, Sandbox };
use iced::{Button, Column, Element, Text};
use native_dialog::FileDialog;

use std::path::PathBuf;

#[derive(Default)]
struct Gui {
    directory: Option<PathBuf>,
    audio_files: Vec<PathBuf>,
    dir_btn_state: button::State,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    SelectDirectory,
}

enum GuiState {
    SelectDirectory,
    SelectFiles,
}

impl Sandbox for Gui {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("Sound Bored")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SelectDirectory => {
                self.directory = FileDialog::new().show_open_single_dir().unwrap()
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::with_children(vec![Button::new(
            &mut self.dir_btn_state,
            Text::new("Choose dir"),
        )
        .on_press(Message::SelectDirectory)
        .into()])
        .into()
    }
}

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
