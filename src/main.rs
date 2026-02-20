mod misc;

use iced::widget::{Column, button, column, row, text, text_input};
use serde::{Deserialize, Serialize};

use crate::misc::{ModalResult, start_analysis};

pub fn main() -> iced::Result {
    iced::run(MainApp::update, MainApp::view)
}

#[derive(Serialize, Deserialize)]
pub struct MainApp {
    omc_path: String,
    model_path: String,
    model_name: String,
    analysis_result: Option<ModalResult>,
    info: String,
}

#[derive(Debug, Clone)]
enum Message {
    SubmitAnalysis,
    ChangedOMCPath(String),
    ChangedModelPath(String),
    ChangedModelName(String),
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            omc_path: "omc".to_string(),
            model_path: "model.mo".to_string(),
            model_name: "model".to_string(),
            analysis_result: None,
            info: String::new(),
        }
    }
}

impl MainApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::SubmitAnalysis => match start_analysis(self) {
                Ok(result) => {
                    println!("          {:?}", &result.eigenfreqs);
                    println!("dampened: {:?}", &result.eigenfreqs_dampened);
                    self.analysis_result = Some(result);
                }
                Err(e) => self.info = e.to_string(),
            },
            Message::ChangedModelPath(input) => self.model_path = input,
            Message::ChangedModelName(input) => self.model_name = input,
            Message::ChangedOMCPath(input) => self.omc_path = input,
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column!(
            row!(
                text("OMC Path"),
                text_input("", &self.omc_path).on_input(Message::ChangedOMCPath)
            ),
            row!(
                text("Model Path"),
                text_input("", &self.model_path).on_input(Message::ChangedModelPath)
            ),
            row!(
                text("Model Name"),
                text_input("", &self.model_name).on_input(Message::ChangedModelName)
            ),
            row!(
                button("Start analysis").on_press(Message::SubmitAnalysis),
                text(&self.info)
            ),
            text("Results").size(24),
        )
    }
}
