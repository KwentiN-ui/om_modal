mod misc;

use iced::{
    Event, Subscription, Task, event,
    widget::{Column, button, column, row, text, text_input},
    window::{self},
};
use serde::{Deserialize, Serialize};

use crate::misc::{ModalResult, start_analysis};

pub const APPNAME: &str = "mo_modal";
pub const CONFIGNAME: &str = "appstate";

pub fn main() -> iced::Result {
    iced::application(MainApp::restore, MainApp::update, MainApp::view)
        .subscription(subscription)
        .exit_on_close_request(false)
        .run()
}

fn subscription(_app: &MainApp) -> Subscription<Message> {
    event::listen_with(|event, _status, _id| match event {
        Event::Window(window::Event::CloseRequested) => Some(Message::Exit),
        _ => None,
    })
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
    Exit,
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
    fn restore() -> Self {
        confy::load(APPNAME, CONFIGNAME).unwrap()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SubmitAnalysis => match start_analysis(self) {
                Ok(result) => {
                    println!("          {:?}", &result.eigenfreqs);
                    println!("dampened: {:?}", &result.eigenfreqs_dampened);
                    self.analysis_result = Some(result);
                    Task::none()
                }
                Err(e) => {
                    self.info = e.to_string();
                    Task::none()
                }
            },
            Message::ChangedModelPath(input) => {
                self.model_path = input;
                Task::none()
            }
            Message::ChangedModelName(input) => {
                self.model_name = input;
                Task::none()
            }
            Message::ChangedOMCPath(input) => {
                self.omc_path = input;
                Task::none()
            }
            Message::Exit => {
                let _ = confy::store(APPNAME, CONFIGNAME, self);
                window::latest().and_then(window::close)
            }
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
