
use relm::{Widget};
use relm_derive::{Msg, widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use pleco::Board;

use super::chess_board_painter::ChessBoardPainter;

#[allow(dead_code)]
pub struct ChessBoardModel {
    size: u32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
    board: Board,
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
            board: Board::start_pos(),
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
        
        self.canvas.connect_draw({
            let board_copy = self.model.board.clone(); 
            move |_source, context| {
                painter.draw_background(context, background_color);
                painter.draw_cells(context, white_cells_color, black_cells_color);
                painter.draw_pieces(context, board_copy.fen().as_str());

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