use gdk::EventMask;
use gtk::prelude::*;
use gtk::Inhibit;
use pleco::Board;
use relm::Widget;
use relm_derive::{widget, Msg};

use super::drag_and_drop_handlers::*;

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
    pub dnd_start_cell_color: (f64, f64, f64),
    pub dnd_end_cell_color: (f64, f64, f64),
    pub dnd_cross_color: (f64, f64, f64),
    pub board: Board,
    pub black_side: BlackSide,
}

#[derive(Default)]
pub struct DndState {
    pub dnd_active: bool,
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub origin_file: u8,
    pub origin_rank: u8,
    pub target_file: u8,
    pub target_rank: u8,
    pub moved_piece_fen: char,
}

#[allow(dead_code)]
pub struct ChessBoardModel {
    chess_state: Rc<RefCell<ChessState>>,
    dnd_state: Rc<RefCell<DndState>>,
}

pub struct ChessStateBuilder {
    size: u32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
    coordinates_color: (f64, f64, f64),
    dnd_start_cell_color: (f64, f64, f64),
    dnd_end_cell_color: (f64, f64, f64),
    dnd_cross_color: (f64, f64, f64),
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
            dnd_start_cell_color: (0.92, 0.12, 0.22),
            dnd_end_cell_color: (0.34, 0.82, 0.14),
            dnd_cross_color: (0.70, 0.18, 0.90),
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
            dnd_start_cell_color: self.dnd_start_cell_color,
            dnd_end_cell_color: self.dnd_end_cell_color,
            dnd_cross_color: self.dnd_cross_color,
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

    fn set_dnd_start_cell_color(&mut self, start_cell_color: (f64, f64, f64)) {
        self.dnd_start_cell_color = start_cell_color;
    }

    fn set_dnd_end_cell_color(&mut self, end_cell_color: (f64, f64, f64)) {
        self.dnd_end_cell_color = end_cell_color;
    }

    fn set_dnd_cross_color(&mut self, cross_color: (f64, f64, f64)) {
        self.dnd_cross_color = cross_color;
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
        let chess_state = Rc::new(RefCell::new(state_builder.build()));
        let dnd_state = Rc::new(RefCell::new(DndState::default()));

        ChessBoardModel {
            chess_state,
            dnd_state,
        }
    }

    fn update(&mut self, event: ChessBoardMsg) {
        match event {
            ChessBoardMsg::SetBlackSide(side) => {
                {
                    let mut chess_state_from_model = (*self.model.chess_state).borrow_mut();
                    (*chess_state_from_model).black_side = side;
                }
                self.repaint();
            }
        }
    }

    fn init_view(&mut self) {
        self.set_canvas_size();
        self.set_canvas_draw_implementation();
        self.add_canvas_mouse_reactivity_implementation();
    }

    view! {
        #[name="canvas"]
        gtk::DrawingArea {
        }
    }
}

impl ChessBoard {
    pub fn repaint(&self) {
        let chess_state = (*self.model.chess_state).borrow();
        let size = chess_state.size;

        self.canvas
            .queue_draw_region(&cairo::Region::create_rectangle(&cairo::RectangleInt {
                x: 0,
                y: 0,
                width: size as i32,
                height: size as i32,
            }));
    }

    pub fn set_canvas_size(&self) {
        let chess_state = (*self.model.chess_state).borrow();
        let size = chess_state.size;
        self.canvas.set_size_request(size as i32, size as i32);
    }

    pub fn build_painter(&self) -> ChessBoardPainter {
        let chess_state = (*self.model.chess_state).borrow();
        let size = chess_state.size;
        let mut painter = ChessBoardPainter::new(size / 9);
        painter.build_images();

        painter
    }

    pub fn set_canvas_draw_implementation(&self) {
        let painter = self.build_painter();
        {
            let weak_chess_state = Rc::downgrade(&self.model.chess_state);
            let weak_dnd_state = Rc::downgrade(&self.model.dnd_state);
            self.canvas.connect_draw(move |_source, context| {
                if let Some(chess_state) = weak_chess_state.upgrade() {
                    if let Some(dnd_state) = weak_dnd_state.upgrade() {
                        let chess_state = chess_state.borrow();
                        let dnd_state = dnd_state.borrow();
                        painter.paint(&context, &chess_state, &dnd_state);
                    }
                }

                Inhibit(false)
            });
        }
    }

    pub fn add_canvas_mouse_reactivity_implementation(&self) {
        self.make_canvas_reactive();
        self.add_canvas_mouse_press_implementation();
        self.add_canvas_mouse_release_implementation();
        self.add_canvas_mouse_move_implementation();
    }

    fn make_canvas_reactive(&self) {
        self.canvas.add_events(
            EventMask::BUTTON_PRESS_MASK
                | EventMask::BUTTON_RELEASE_MASK
                | EventMask::POINTER_MOTION_MASK,
        );
    }

    fn add_canvas_mouse_press_implementation(&self) {
        let weak_chess_state = Rc::downgrade(&self.model.chess_state);
        let weak_dnd_state = Rc::downgrade(&self.model.dnd_state);

        self.canvas
            .connect_button_press_event(move |canvas, event| {
                if let Some(dnd_state) = weak_dnd_state.upgrade() {
                    if let Some(chess_state) = weak_chess_state.upgrade() {
                        let dnd_state = &(*dnd_state);
                        let chess_state = &(*chess_state);
                        mouse_pressed_handler(dnd_state, chess_state, canvas, event);
                    }
                }
                Inhibit(false)
            });
    }

    fn add_canvas_mouse_release_implementation(&self) {
        let weak_chess_state = Rc::downgrade(&self.model.chess_state);
        let weak_dnd_state = Rc::downgrade(&self.model.dnd_state);

        self.canvas
            .connect_button_release_event(move |canvas, event| {
                if let Some(dnd_state) = weak_dnd_state.upgrade() {
                    if let Some(chess_state) = weak_chess_state.upgrade() {
                        let dnd_state = &(*dnd_state);
                        let chess_state = &(*chess_state);
                        mouse_released_handler(dnd_state, chess_state, canvas, event);
                    }
                }
                Inhibit(false)
            });
    }

    fn add_canvas_mouse_move_implementation(&self) {
        let weak_chess_state = Rc::downgrade(&self.model.chess_state);
        let weak_dnd_state = Rc::downgrade(&self.model.dnd_state);

        self.canvas
            .connect_motion_notify_event(move |canvas, event| {
                if let Some(dnd_state) = weak_dnd_state.upgrade() {
                    if let Some(chess_state) = weak_chess_state.upgrade() {
                        let dnd_state = &(*dnd_state);
                        let chess_state = &(*chess_state);
                        mouse_moved_handler(dnd_state, chess_state, canvas, event);
                    }
                }
                Inhibit(false)
            });
    }
}