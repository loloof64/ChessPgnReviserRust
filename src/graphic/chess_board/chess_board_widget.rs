
use relm::{Widget};
use relm_derive::{Msg, widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use pleco::Board;

use std::rc::Rc;
use std::cell::RefCell;

use super::chess_board_painter::ChessBoardPainter;

#[allow(dead_code)]
pub struct ChessBoardModel {
    size: u32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
    board: Rc<RefCell<Board>>,
}

pub struct ChessBoardModelBuilder {
    size: u32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
}

#[allow(dead_code)]
impl ChessBoardModelBuilder {
    fn new() -> Self {
        ChessBoardModelBuilder {
            size: 300,
            background_color: (0.5, 0.4, 0.9),
            white_cells_color: (1.0, 0.85, 0.6),
            black_cells_color: (0.85, 0.55, 0.25),
        }
    }

    fn build(self) -> ChessBoardModel {
        ChessBoardModel {
            size: self.size,
            background_color: self.background_color,
            white_cells_color: self.white_cells_color,
            black_cells_color: self.black_cells_color,
            board: Rc::new(RefCell::new(Board::start_pos())),
        }
    }

    fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    fn set_background_color(&mut self, background_color: (f64, f64, f64)) {
        self.background_color = background_color;
    }

    fn set_white_cells_color(&mut self, white_cells_color: (f64, f64, f64)) {
        self.white_cells_color = white_cells_color;
    }

    fn set_black_cells_color(&mut self, black_cells_color: (f64, f64, f64)) {
        self.black_cells_color = black_cells_color;
    }
}

#[derive(Msg)]
pub enum ChessBoardMsg {
    SetEndgame(),
}

#[widget]
impl Widget for ChessBoard {
    fn model(size: u32) -> ChessBoardModel {
        let mut model_builder = ChessBoardModelBuilder::new();
        model_builder.set_size(size);
        model_builder.build()
    }

    fn update(&mut self, event: ChessBoardMsg) {
        match event {
            ChessBoardMsg::SetEndgame() => {
                {
                    let new_board = Board::from_fen("8/8/3k4/8/3K4/8/3P4/8 w - - 0 34")
                        .expect("failed to get board new position");
                    let mut board_from_model = (*self.model.board).borrow_mut();
                    *board_from_model = new_board;
                }
                self.repaint();
            },
        }
    }

    fn init_view(&mut self) {
        self.canvas.set_size_request(self.model.size as i32, self.model.size as i32);

        let background_color = self.model.background_color;
        let white_cells_color = self.model.white_cells_color;
        let black_cells_color = self.model.black_cells_color;
        let size = self.model.size;

        let mut painter = ChessBoardPainter::new(size / 9);
        painter.build_images();
        
        let board = Rc::clone(&self.model.board);
        self.canvas.connect_draw({
            move |_source, context| {
                let board = board.borrow();
                
                painter.draw_background(context, background_color);
                painter.draw_cells(context, white_cells_color, black_cells_color);
                painter.draw_pieces(context, board.fen().as_str());
                
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
        self.canvas.queue_draw_region(
            &cairo::Region::create_rectangle(&cairo::RectangleInt{
                x: 0, y: 0, width: self.model.size as i32, height: self.model.size as i32, 
            })
        );
    }
}
