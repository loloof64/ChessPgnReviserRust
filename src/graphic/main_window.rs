use gtk::prelude::*;
use gtk::Inhibit;
use relm::Widget;
use relm_derive::{widget, Msg};

use super::chess_board::*;

pub struct WinModel {}

#[derive(Msg)]
pub enum WinMsg {
    Quit,
    SetEndgame,
}

#[widget]
impl Widget for Win {
    fn model() -> WinModel {
        WinModel {}
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            WinMsg::Quit => gtk::main_quit(),
            WinMsg::SetEndgame => self.chess_board.emit(ChessBoardMsg::SetEndgame()),
        }
    }

    view! {
        gtk::Window {
            gtk::Box(gtk::Orientation::Vertical, 5) {
                #[name="chess_board"]
                ChessBoard(500) {

                },
                gtk::Button {
                    label: "set endgame",
                    clicked() => Some(WinMsg::SetEndgame),
                },
            },
            delete_event(_self, _event) => (WinMsg::Quit, Inhibit(false)),
        }
    }
}

pub fn start() {
    Win::run(()).expect("Failed to launch main window");
}
