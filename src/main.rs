mod misc;

use iced::{
    Event, Subscription, Task, event,
    widget::{Column, button, column, row, table, text, text_input},
    window::{self},
};
use serde::{Deserialize, Serialize};

use crate::misc::{EigenMode, start_analysis};

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
    modes: Option<Vec<EigenMode>>,
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
            modes: None,
            info: String::new(),
        }
    }
}

impl MainApp {
    fn restore() -> Self {
        confy::load(APPNAME, CONFIGNAME).unwrap_or_default()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SubmitAnalysis => match start_analysis(self) {
                Ok(result) => {
                    self.modes = Some(result);
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
        let table = {
            let columns = [
                table::column("i", |mode: &EigenMode| text(mode.i)),
                table::column("Eigenvalue", |mode: &EigenMode| {
                    text!("{:.6} ± {:.6}j", mode.re, mode.im)
                }),
                table::column("f [Hz]", |mode: &EigenMode| text!("{:.6}", mode.eigenfreq)),
                table::column("f (dampened) [Hz]", |mode: &EigenMode| {
                    text!("{:.6}", mode.eigenfreq_dampened)
                }),
            ];
            table(columns, self.modes.as_deref().unwrap_or(&[]))
        };

        column!(
            row!(
                text("OMC Path"),
                text_input("", &self.omc_path).on_input(Message::ChangedOMCPath)
            )
            .spacing(20.),
            row!(
                text("Model Path"),
                text_input("", &self.model_path).on_input(Message::ChangedModelPath)
            )
            .spacing(20.),
            row!(
                text("Model Name"),
                text_input("", &self.model_name).on_input(Message::ChangedModelName)
            )
            .spacing(20.),
            row!(
                button("Start analysis").on_press(Message::SubmitAnalysis),
                text(&self.info)
            )
            .spacing(20.),
            text("Results").size(24),
            table
        )
        .spacing(20.)
        .padding(20.)
    }
}
