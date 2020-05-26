use gtk::prelude::*;
use gtk::Inhibit;
use relm::Widget;
use relm_derive::{widget, Msg};

use super::chess_board::*;

pub struct WinModel {
    black_side: BlackSide,
}

#[derive(Msg)]
pub enum WinMsg {
    Quit,
    SetBoardUpsideDown,
}

#[widget]
impl Widget for Win {
    fn model() -> WinModel {
        WinModel {
            black_side: BlackSide::BlackTop,
        }
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            WinMsg::Quit => gtk::main_quit(),
            WinMsg::SetBoardUpsideDown => {
                let new_black_side = match self.model.black_side {
                    BlackSide::BlackBottom => BlackSide::BlackTop,
                    BlackSide::BlackTop => BlackSide::BlackBottom,
                };
                self.model.black_side = new_black_side;
                self.chess_board
                    .emit(ChessBoardMsg::SetBlackSide(new_black_side));
            }
        }
    }

    view! {
        gtk::Window {
            gtk::Box(gtk::Orientation::Vertical, 5) {
                #[name="chess_board"]
                ChessBoard(500) {

                },
                gtk::Button {
                    label: "Toggle board orientation",
                    clicked() => Some(WinMsg::SetBoardUpsideDown),
                },
            },
            delete_event(_self, _event) => (WinMsg::Quit, Inhibit(false)),
        }
    }
}

pub fn start() {
    Win::run(()).expect("Failed to launch main window");
}
