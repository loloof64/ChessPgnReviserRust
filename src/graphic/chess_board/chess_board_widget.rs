use gtk::prelude::*;
use gtk::Inhibit;
use pleco::Board;
use relm::Widget;
use relm_derive::{widget, Msg};

use std::cell::RefCell;
use std::rc::Rc;

use super::chess_board_painter::ChessBoardPainter;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BlackSide {
    BlackTop,
    BlackBottom,
}

#[allow(dead_code)]
pub struct ChessBoardModel {
    size: u32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
    coordinates_color: (f64, f64, f64),
    board: Rc<RefCell<Board>>,
    black_side: Rc<RefCell<BlackSide>>,
}

pub struct ChessBoardModelBuilder {
    size: u32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
    coordinates_color: (f64, f64, f64),
    black_side: BlackSide,
}

#[allow(dead_code)]
impl ChessBoardModelBuilder {
    fn new() -> Self {
        ChessBoardModelBuilder {
            size: 300,
            background_color: (0.5, 0.4, 0.9),
            white_cells_color: (1.0, 0.85, 0.6),
            black_cells_color: (0.85, 0.55, 0.25),
            coordinates_color: (1.0, 0.78, 0.0),
            black_side: BlackSide::BlackTop,
        }
    }

    fn build(self) -> ChessBoardModel {
        ChessBoardModel {
            size: self.size,
            background_color: self.background_color,
            white_cells_color: self.white_cells_color,
            black_cells_color: self.black_cells_color,
            coordinates_color: self.coordinates_color,
            black_side: Rc::new(RefCell::new(self.black_side)),
            board: Rc::new(RefCell::new(Board::start_pos())),
        }
    }

    fn set_board_size(&mut self, size: u32) {
        self.size = size;
    }

    fn set_board_background_color(&mut self, background_color: (f64, f64, f64)) {
        self.background_color = background_color;
    }

    fn set_board_white_cells_color(&mut self, white_cells_color: (f64, f64, f64)) {
        self.white_cells_color = white_cells_color;
    }

    fn set_board_black_cells_color(&mut self, black_cells_color: (f64, f64, f64)) {
        self.black_cells_color = black_cells_color;
    }

    fn set_board_coordinates_color(&mut self, coordinates_color: (f64, f64, f64)) {
        self.coordinates_color = coordinates_color;
    }

    fn set_board_orientation(&mut self, side: BlackSide) {
        self.black_side = side;
    }
}

#[derive(Msg)]
pub enum ChessBoardMsg {
    SetBlackSide(BlackSide),
}

#[widget]
impl Widget for ChessBoard {
    fn model(board_size: u32) -> ChessBoardModel {
        let mut model_builder = ChessBoardModelBuilder::new();
        model_builder.set_board_size(board_size);
        model_builder.build()
    }

    fn update(&mut self, event: ChessBoardMsg) {
        match event {
            ChessBoardMsg::SetBlackSide(side) => {
                {
                    let mut black_side_from_model = (*self.model.black_side).borrow_mut();
                    *black_side_from_model = side;
                }
                self.repaint();
            }
        }
    }

    fn init_view(&mut self) {
        self.canvas
            .set_size_request(self.model.size as i32, self.model.size as i32);

        let background_color = self.model.background_color;
        let white_cells_color = self.model.white_cells_color;
        let black_cells_color = self.model.black_cells_color;
        let coordinates_color = self.model.coordinates_color;
        let size = self.model.size;
        
        let mut painter = ChessBoardPainter::new(size / 9);
        painter.build_images();
        let board = Rc::clone(&self.model.board);
        let black_side = Rc::clone(&self.model.black_side);

        self.canvas.connect_draw({
            move |_source, context| {
                let black_side = (*black_side).borrow();

                let board = board.borrow();
                painter.draw_background(context, background_color);
                painter.draw_cells(context, white_cells_color, black_cells_color);
                painter.draw_player_turn(context, board.fen().as_str());
                painter.draw_pieces(context, board.fen().as_str(), *black_side);
                painter.draw_coordinates(context, coordinates_color, *black_side);
                Inhibit(true)
            }
        });
    }

    view! {
        #[name="canvas"]
        gtk::DrawingArea {
        }
    }
}

impl ChessBoard {
    pub fn repaint(&self) {
        self.canvas
            .queue_draw_region(&cairo::Region::create_rectangle(&cairo::RectangleInt {
                x: 0,
                y: 0,
                width: self.model.size as i32,
                height: self.model.size as i32,
            }));
    }
}
