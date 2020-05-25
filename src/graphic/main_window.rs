use relm::{Widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use relm_derive::{Msg, widget};

use super::chess_board::*;

pub struct WinModel {

}

#[derive(Msg)]
pub enum WinMsg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> WinModel {
        WinModel{}
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            WinMsg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            #[name="chess_board"]
            ChessBoard(200) {

            },
            delete_event(_, _) => (WinMsg::Quit, Inhibit(false)),
        }
    }
}

pub fn start() {
    Win::run(()).expect("Failed to launch main window");
}