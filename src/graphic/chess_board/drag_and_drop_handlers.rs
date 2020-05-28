use gdk::{EventButton, EventMotion};
use gtk::prelude::*;
use gtk::DrawingArea;
use pleco::core::sq::SQ;
use std::cell::RefCell;

use super::chess_board_widget::*;

pub fn mouse_pressed_handler(
    dnd_state: &RefCell<DndState>,
    chess_state: &RefCell<ChessState>,
    canvas: &DrawingArea,
    event: &EventButton,
) {
    if !get_dnd_active_state(dnd_state) {
        let (x, y) = event.get_position();
        let file = get_file(x, chess_state);
        let rank = get_rank(y, chess_state);
        
        if cell_in_bounds(file, rank) {
            update_cursor_position(x, y, chess_state, dnd_state);

            let file = file as u8;
            let rank = rank as u8;

            if let Some(fen) = piece_at_square(file, rank, chess_state) {
                set_dnd_active(fen, file, rank, dnd_state);
                repaint_canvas(canvas, chess_state);
            }
        }
    }
}

pub fn mouse_released_handler(
    dnd_state: &RefCell<DndState>,
    chess_state: &RefCell<ChessState>,
    canvas: &DrawingArea,
    event: &EventButton,
) {
    if get_dnd_active_state(dnd_state) {
        set_dnd_inactive(dnd_state);
        let (x, y) = event.get_position();
        update_cursor_position(x, y, chess_state, dnd_state);
        repaint_canvas(canvas, chess_state);
    }
}

pub fn mouse_moved_handler(
    dnd_state: &RefCell<DndState>,
    chess_state: &RefCell<ChessState>,
    canvas: &DrawingArea,
    event: &EventMotion,
) {
    if get_dnd_active_state(dnd_state) {
        let (x, y) = event.get_position();
        update_cursor_position(x, y, chess_state, dnd_state);
        repaint_canvas(canvas, chess_state);
    }
}

fn get_dnd_active_state(dnd_state: &RefCell<DndState>) -> bool {
    let dnd_state = (*dnd_state).borrow();
    dnd_state.dnd_active
}

fn repaint_canvas(canvas: &DrawingArea, chess_state: &RefCell<ChessState>) {
    let chess_state = chess_state.borrow();
    let canvas_size = chess_state.size as i32;

    canvas.queue_draw_region(&cairo::Region::create_rectangle(&cairo::RectangleInt {
        x: 0,
        y: 0,
        width: canvas_size,
        height: canvas_size,
    }));
}

fn update_cursor_position(
    x: f64,
    y: f64,
    chess_state: &RefCell<ChessState>,
    dnd_state: &RefCell<DndState>,
) {
    let chess_state = chess_state.borrow();
    let mut dnd_state = dnd_state.borrow_mut();

    let size = chess_state.size;
    let cells_size = size as f64 / 9_f64;

    dnd_state.cursor_x = x - cells_size * 0.5;
    dnd_state.cursor_y = y - cells_size * 0.5;
}

fn set_dnd_active(value: char, origin_file: u8, origin_rank: u8, dnd_state: &RefCell<DndState>) {
    let mut dnd_state = dnd_state.borrow_mut();
    dnd_state.origin_file = origin_file;
    dnd_state.origin_rank = origin_rank;

    dnd_state.moved_piece_fen = value;
    dnd_state.dnd_active = true;
}

fn set_dnd_inactive(dnd_state: &RefCell<DndState>) {
    let mut dnd_state = dnd_state.borrow_mut();
    dnd_state.dnd_active = false;
}

fn get_file(x: f64, chess_state: &RefCell<ChessState>) -> i8 {
    let chess_state = chess_state.borrow();
    let cells_size = chess_state.size as f64 / 9_f64;
    let black_side = chess_state.black_side;

    let col = ((x - cells_size * 0.5) / cells_size) as i8;

    if black_side == BlackSide::BlackBottom {
        7 - col
    } else {
        col
    }
}

fn get_rank(y: f64, chess_state: &RefCell<ChessState>) -> i8 {
    let chess_state = chess_state.borrow();
    let cells_size = chess_state.size as f64 / 9_f64;
    let black_side = chess_state.black_side;

    let row = ((y - cells_size * 0.5) / cells_size) as i8;

    if black_side == BlackSide::BlackBottom {
        row
    } else {
        7 - row
    }
}

fn cell_in_bounds(file: i8, rank: i8) -> bool {
    file >= 0 && file <= 7 && rank >= 0 && rank <= 7
}

fn piece_at_square(file: u8, rank: u8, chess_state: &RefCell<ChessState>) -> Option<char> {
    let chess_state = chess_state.borrow();
    chess_state
        .board
        .piece_at_sq(SQ::from(rank * 8 + file))
        .character()
}
