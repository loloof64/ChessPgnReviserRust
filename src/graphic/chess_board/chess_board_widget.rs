use gdk::EventMask;
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

pub struct ChessState {
    pub size: u32,
    pub background_color: (f64, f64, f64),
    pub white_cells_color: (f64, f64, f64),
    pub black_cells_color: (f64, f64, f64),
    pub coordinates_color: (f64, f64, f64),
    pub board: Board,
    pub black_side: BlackSide,
}

#[allow(dead_code)]
pub struct ChessBoardModel {
    state: Rc<RefCell<ChessState>>,
}

pub struct ChessStateBuilder {
    size: u32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
    coordinates_color: (f64, f64, f64),
    black_side: BlackSide,
}

#[allow(dead_code)]
impl ChessStateBuilder {
    fn new() -> Self {
        ChessStateBuilder {
            size: 300,
            background_color: (0.5, 0.4, 0.9),
            white_cells_color: (1.0, 0.85, 0.6),
            black_cells_color: (0.85, 0.55, 0.25),
            coordinates_color: (1.0, 0.78, 0.0),
            black_side: BlackSide::BlackTop,
        }
    }

    fn build(self) -> ChessState {
        ChessState {
            size: self.size,
            background_color: self.background_color,
            white_cells_color: self.white_cells_color,
            black_cells_color: self.black_cells_color,
            coordinates_color: self.coordinates_color,
            black_side: self.black_side,
            board: Board::start_pos(),
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
        let mut state_builder = ChessStateBuilder::new();
        state_builder.set_board_size(board_size);
        let state = Rc::new(RefCell::new(state_builder.build()));

        ChessBoardModel { state }
    }

    fn update(&mut self, event: ChessBoardMsg) {
        match event {
            ChessBoardMsg::SetBlackSide(side) => {
                {
                    let mut state_from_model = (*self.model.state).borrow_mut();
                    (*state_from_model).black_side = side;
                }
                self.repaint();
            }
        }
    }

    fn init_view(&mut self) {
        self.set_canvas_size();
        self.canvas.add_events(
            EventMask::BUTTON_PRESS_MASK
                | EventMask::BUTTON_RELEASE_MASK
                | EventMask::POINTER_MOTION_MASK,
        );

        self.set_canvas_draw_implementation();
    }

    view! {
        #[name="canvas"]
        gtk::DrawingArea {
        }
    }
}

impl ChessBoard {
    pub fn repaint(&self) {
        let state = (*self.model.state).borrow();
        let size = state.size;

        self.canvas
            .queue_draw_region(&cairo::Region::create_rectangle(&cairo::RectangleInt {
                x: 0,
                y: 0,
                width: size as i32,
                height: size as i32,
            }));
    }

    pub fn set_canvas_size(&self) {
        let state = (*self.model.state).borrow();
        let size = state.size;
        self.canvas.set_size_request(size as i32, size as i32);
    }

    pub fn build_painter(&self) -> ChessBoardPainter {
        let state = (*self.model.state).borrow();
        let size = state.size;
        let mut painter = ChessBoardPainter::new(size / 9);
        painter.build_images();

        painter
    }

    pub fn set_canvas_draw_implementation(&self) {
        let painter = self.build_painter();
        {
            let weak_state = Rc::downgrade(&self.model.state);
            self.canvas.connect_draw(move |_source, context| {
                if let Some(state) = weak_state.upgrade() {
                    let state = state.borrow();
                    painter.paint(&context, &state);
                }

                Inhibit(false)
            });
        }
    }
}
