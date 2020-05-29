use failure::Fail;

use cairo::{Context, FontFace, FontWeight, ImageSurface};
use resvg::backend_cairo::render_to_image;
use resvg::usvg::ShapeRendering;
use resvg::{usvg::Tree, FitTo, Options};
use std::collections::HashMap;

use super::chess_board_widget::{BlackSide, ChessState, DndState};

#[derive(Debug, Fail)]
pub enum ChessPiecesError {
    #[fail(display = "Bad piece fen: {}", fen)]
    BadPieceFenReference { fen: char },
}

struct ChessPiecesImages {
    images: HashMap<char, ImageSurface>,
}

impl ChessPiecesImages {
    fn new() -> Self {
        ChessPiecesImages {
            images: HashMap::new(),
        }
    }

    fn build_images(&mut self, cells_size: u32) {
        let options = ChessPiecesImages::svg_options_for_cells_size(cells_size);

        for fen in vec!['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'] {
            let svg_content = ChessPiecesImages::piece_value_to_svg_definition(fen, &options);
            let image_to_render =
                ChessPiecesImages::image_surface_from_svg_definition(&svg_content, &options);

            self.insert_image_if_defined(fen, image_to_render);
        }
    }

    fn insert_image_if_defined(&mut self, fen: char, image: Option<ImageSurface>) {
        if let Some(image) = image {
            self.images.insert(fen, image);
        }
    }

    fn image_surface_from_svg_definition(
        svg_tree: &Option<Tree>,
        options: &Options,
    ) -> Option<ImageSurface> {
        if let Some(tree) = svg_tree {
            if let Some(image) = render_to_image(&tree, &options) {
                Some(image)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn svg_options_for_cells_size(cells_size: u32) -> Options {
        let base_size = 45f32;
        let scale = cells_size as f32 / base_size;
        let mut options = Options::default();
        options.usvg.shape_rendering = ShapeRendering::GeometricPrecision;
        options.fit_to = FitTo::Zoom(scale);

        options
    }

    fn piece_value_to_svg_definition(piece_value_fen: char, options: &Options) -> Option<Tree> {
        match piece_value_fen {
            'P' => Some(include_str!("./Chess_plt45.svg")),
            'N' => Some(include_str!("./Chess_nlt45.svg")),
            'B' => Some(include_str!("./Chess_blt45.svg")),
            'R' => Some(include_str!("./Chess_rlt45.svg")),
            'Q' => Some(include_str!("./Chess_qlt45.svg")),
            'K' => Some(include_str!("./Chess_klt45.svg")),
            'p' => Some(include_str!("./Chess_pdt45.svg")),
            'n' => Some(include_str!("./Chess_ndt45.svg")),
            'b' => Some(include_str!("./Chess_bdt45.svg")),
            'r' => Some(include_str!("./Chess_rdt45.svg")),
            'q' => Some(include_str!("./Chess_qdt45.svg")),
            'k' => Some(include_str!("./Chess_kdt45.svg")),
            _ => None,
        }
        .map(|file_content| Tree::from_data(file_content.as_bytes(), &options.usvg).ok())
        .unwrap_or(None)
    }

    fn get_image_for_fen(&self, fen: char) -> Result<ImageSurface, ChessPiecesError> {
        match self.images.get(&fen) {
            Some(image) => Ok((image).clone()),
            None => Err(ChessPiecesError::BadPieceFenReference { fen }),
        }
    }
}

pub struct ChessBoardPainter {
    cells_size: u32,
    pieces_images: ChessPiecesImages,
}

impl ChessBoardPainter {
    pub fn new(cells_size: u32) -> Self {
        ChessBoardPainter {
            cells_size,
            pieces_images: ChessPiecesImages::new(),
        }
    }

    pub fn build_images(&mut self) {
        self.pieces_images.build_images(self.cells_size);
    }

    pub fn paint(&self, context: &Context, chess_state: &ChessState, dnd_state: &DndState) {
        self.draw_coordinates(context, chess_state);
        self.draw_player_turn(context, chess_state);
        self.draw_background(context, chess_state);
        self.draw_cells(context, chess_state, dnd_state);
        self.draw_pieces(context, chess_state, dnd_state);
        self.draw_last_move(context, chess_state);
        self.draw_cursor_piece(context, dnd_state);
    }

    fn draw_background(&self, context: &Context, chess_state: &ChessState) {
        let background_color = chess_state.background_color;
        let (bg_red, bg_green, bg_blue) = background_color;

        context.set_source_rgb(bg_red, bg_green, bg_blue);
        context.paint();
    }

    fn draw_cells(&self, context: &Context, chess_state: &ChessState, dnd_state: &DndState) {
        let cells_size = self.cells_size as f64;

        for row in 0..8 {
            for col in 0..8 {
                setup_current_cell_color(context, col, row, chess_state);
                setup_dnd_highlight_if_matches(context, col, row, chess_state, dnd_state);
                fill_current_cell_to_setup(context, col, row, cells_size);
            }
        }
    }

    fn draw_pieces(&self, context: &Context, chess_state: &ChessState, dnd_state: &DndState) {
        let position = chess_state.board.fen();
        let position = position.as_str();
        let black_side = chess_state.black_side;
        let pieces_lines = self.get_pieces_values_from_fen(position);

        for (line_index, line) in pieces_lines.iter().enumerate() {
            self.draw_pieces_line(context, line, line_index as u8, black_side, dnd_state);
        }
    }

    fn draw_player_turn(&self, context: &Context, chess_state: &ChessState) {
        let position = chess_state.board.fen();
        let position = position.as_str();
        let turn_str = position.split(" ").skip(1).take(1).collect::<Vec<_>>()[0];
        let is_white_turn = turn_str == "w";

        let cells_size = self.cells_size as f64;
        let location = cells_size * 8.75;
        let radius = cells_size * 0.25;
        let color = if is_white_turn {
            (1f64, 1f64, 1f64)
        } else {
            (0f64, 0f64, 0f64)
        };

        context.set_source_rgb(color.0, color.1, color.2);
        context.arc(
            location,
            location,
            radius,
            0f64,
            2f64 * std::f64::consts::PI,
        );
        context.fill();
    }

    fn draw_last_move(&self, context: &Context, chess_state: &ChessState) {
        if let Some(last_move) = &chess_state.last_move {
            let (lm_red, lm_green, lm_blue) = chess_state.last_move_arrow_color;
            let alpha = 0.7;
            context.set_source_rgba(lm_red, lm_green, lm_blue, alpha);
            context.set_line_width(self.cells_size as f64 * 0.10);

            let origin_file = last_move.origin.file as f64;
            let origin_rank = last_move.origin.rank as f64;

            let target_file = last_move.target.file as f64;
            let target_rank = last_move.target.rank as f64;

            let cells_size = self.cells_size as f64;

            let origin_x = if chess_state.black_side == BlackSide::BlackBottom {
                cells_size * (8.0 - origin_file)
            } else {
                cells_size * (1.0 + origin_file)
            };

            let origin_y = if chess_state.black_side == BlackSide::BlackBottom {
                cells_size * (1.0 + origin_rank)
            } else {
                cells_size * (8.0 - origin_rank)
            };

            let target_x = if chess_state.black_side == BlackSide::BlackBottom {
                cells_size * (8.0 - target_file)
            } else {
                cells_size * (1.0 + target_file)
            };

            let target_y = if chess_state.black_side == BlackSide::BlackBottom {
                cells_size * (1.0 + target_rank)
            } else {
                cells_size * (8.0 - target_rank)
            };

            self.draw_arrow(
                context,
                origin_x,
                origin_y,
                target_x,
                target_y,
                cells_size * 0.5,
                cells_size * 0.5,
            );
        }
    }

    fn draw_arrow(
        &self,
        context: &Context,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        arrow_length: f64,
        arrow_width: f64,
    ) {
        // Inspired by algorithm at
        // http://xymaths.free.fr/Informatique-Programmation/javascript/canvas-dessin-fleche.php

        let dx = x2 - x1;
        let dy = y2 - y1;
        let base_length = (dx * dx + dy * dy).sqrt();

        let shaft_x = x2 + arrow_length * (x1 - x2) / base_length;
        let shaft_y = y2 + arrow_length * (y1 - y2) / base_length;

        let arrow_1_x = shaft_x + arrow_width * (y1 - y2) / base_length;
        let arrow_1_y = shaft_y + arrow_width * (x2 - x1) / base_length;

        let arrow_2_x = shaft_x - arrow_width * (y1 - y2) / base_length;
        let arrow_2_y = shaft_y - arrow_width * (x2 - x1) / base_length;

        context.move_to(x1, y1);
        context.line_to(x2, y2);
        context.stroke();

        context.move_to(arrow_1_x, arrow_1_y);
        context.line_to(x2, y2);
        context.line_to(arrow_2_x, arrow_2_y);
        context.stroke();
    }

    fn draw_coordinates(&self, context: &Context, chess_state: &ChessState) {
        let coordinates_color = chess_state.coordinates_color;

        self.prepare_coordinates_drawing(&context, coordinates_color);
        self.draw_files_coordinates(&context, &chess_state);
        self.draw_ranks_coordinates(&context, &chess_state);
    }

    fn draw_cursor_piece(&self, context: &Context, dnd_state: &DndState) {
        if dnd_state.dnd_active {
            let image = self.get_image_cursor_for_fen(dnd_state.moved_piece_fen);
            self.draw_single_piece_image(context, image, dnd_state.cursor_x, dnd_state.cursor_y);
        }
    }

    fn get_image_cursor_for_fen(&self, fen: char) -> ImageSurface {
        let original_image = &self.pieces_images.images[&fen];
        original_image.clone()
    }

    fn get_pieces_values_from_fen<'a>(&self, position_fen: &'a str) -> Vec<&'a str> {
        let board_value = position_fen.split(" ").take(1).collect::<Vec<_>>()[0];
        board_value.split("/").collect::<Vec<_>>()
    }

    fn draw_pieces_line(
        &self,
        context: &Context,
        line: &str,
        line_index: u8,
        black_side: BlackSide,
        dnd_state: &DndState,
    ) {
        let line_values = line.chars();
        let mut col_index = 0_u8;
        let ascii_0 = 48;
        let ascii_9 = 57;

        for value in line_values {
            let value_ascii = value as u8;
            let is_digit_value = value_ascii >= ascii_0 && value_ascii <= ascii_9;

            if is_digit_value {
                col_index += self.skip_and_count_holes(value_ascii);
            } else {
                self.draw_single_piece_if_not_moved_one(
                    context, value, col_index, line_index, black_side, dnd_state,
                );
                col_index += 1;
            }
        }
    }

    fn skip_and_count_holes(&self, value_ascii: u8) -> u8 {
        let ascii_0 = 48;
        let holes_count = value_ascii - ascii_0;
        holes_count
    }

    fn draw_single_piece_if_not_moved_one(
        &self,
        context: &Context,
        value: char,
        col_index: u8,
        line_index: u8,
        black_side: BlackSide,
        dnd_state: &DndState,
    ) {
        let file = col_index;
        let rank = 7 - line_index;
        let is_not_moved_piece_cell =
            !dnd_state.dnd_active || dnd_state.origin_file != file || dnd_state.origin_rank != rank;

        if is_not_moved_piece_cell {
            self.draw_single_piece(context, value, col_index, line_index, black_side);
        }
    }

    fn draw_single_piece(
        &self,
        context: &Context,
        value: char,
        col_index: u8,
        line_index: u8,
        black_side: BlackSide,
    ) {
        let col = self.get_col(col_index, black_side) as f64;
        let row = self.get_row(line_index, black_side) as f64;
        let cells_size = self.cells_size as f64;

        self.draw_single_piece_image(
            context,
            self.pieces_images
                .get_image_for_fen(value)
                .expect(format!("could not get image for {}", value).as_str()),
            cells_size * (0.5 + col),
            cells_size * (0.5 + row),
        );
    }

    fn get_col(&self, col_index: u8, black_side: BlackSide) -> u8 {
        if black_side == BlackSide::BlackBottom {
            7 - col_index
        } else {
            col_index
        }
    }

    fn get_row(&self, line_index: u8, black_side: BlackSide) -> u8 {
        if black_side == BlackSide::BlackBottom {
            7 - line_index
        } else {
            line_index
        }
    }

    fn prepare_coordinates_drawing(&self, context: &Context, coordinates_color: (f64, f64, f64)) {
        let cells_size = self.cells_size as f64;
        let font_size = 0.35 * cells_size;
        let old_font_face = context.get_font_face();

        context.set_source_rgb(
            coordinates_color.0,
            coordinates_color.1,
            coordinates_color.2,
        );
        context.set_font_size(font_size);
        context.set_font_face(&FontFace::toy_create(
            old_font_face
                .toy_get_family()
                .expect("Failed to get current font family")
                .as_str(),
            old_font_face.toy_get_slant(),
            FontWeight::Bold,
        ));
    }

    fn draw_files_coordinates(&self, context: &Context, chess_state: &ChessState) {
        let cells_size = self.cells_size as f64;
        let ascii_uppercase_a = 65u8;
        let black_side = chess_state.black_side;

        for col in 0..8 {
            let file = if black_side == BlackSide::BlackBottom {
                7 - col
            } else {
                col
            };
            let coordinate = (ascii_uppercase_a + file) as char;
            let coordinate = format!("{}", coordinate);
            let coordinate = coordinate.as_str();

            let x = cells_size * (0.9 + col as f64);
            let y1 = cells_size * 0.35;
            let y2 = cells_size * 8.85;

            context.move_to(x, y1);
            context.show_text(coordinate);

            context.move_to(x, y2);
            context.show_text(coordinate);
        }
    }

    fn draw_ranks_coordinates(&self, context: &Context, chess_state: &ChessState) {
        let cells_size = self.cells_size as f64;
        let ascii_1 = 49u8;
        let black_side = chess_state.black_side;

        for row in 0..8 {
            let rank = if black_side == BlackSide::BlackBottom {
                row
            } else {
                7 - row
            };
            let coordinate = (ascii_1 + rank) as char;
            let coordinate = format!("{}", coordinate);
            let coordinate = coordinate.as_str();

            let y = cells_size * (1.2 + row as f64);
            let x1 = cells_size * 0.15;
            let x2 = cells_size * 8.65;

            context.move_to(x1, y);
            context.show_text(coordinate);

            context.move_to(x2, y);
            context.show_text(coordinate);
        }
    }

    fn draw_single_piece_image(&self, context: &Context, image: ImageSurface, x: f64, y: f64) {
        let origin = 0f64;

        context.save();
        context.translate(x, y);
        context.set_source_surface(&image, origin, origin);
        context.paint();
        context.fill();
        context.restore();
    }
}

fn is_dnd_start_cell(col: i8, row: i8, chess_state: &ChessState, dnd_state: &DndState) -> bool {
    let file = if chess_state.black_side == BlackSide::BlackBottom {
        7 - col
    } else {
        col
    } as u8;
    let rank = if chess_state.black_side == BlackSide::BlackBottom {
        row
    } else {
        7 - row
    } as u8;

    file == dnd_state.origin_file && rank == dnd_state.origin_rank
}

fn is_dnd_target_cell(col: i8, row: i8, chess_state: &ChessState, dnd_state: &DndState) -> bool {
    let file = if chess_state.black_side == BlackSide::BlackBottom {
        7 - col
    } else {
        col
    } as u8;
    let rank = if chess_state.black_side == BlackSide::BlackBottom {
        row
    } else {
        7 - row
    } as u8;

    file == dnd_state.target_file && rank == dnd_state.target_rank
}

fn is_dnd_cross_cell(col: i8, row: i8, chess_state: &ChessState, dnd_state: &DndState) -> bool {
    let file = if chess_state.black_side == BlackSide::BlackBottom {
        7 - col
    } else {
        col
    } as u8;
    let rank = if chess_state.black_side == BlackSide::BlackBottom {
        row
    } else {
        7 - row
    } as u8;

    file == dnd_state.target_file || rank == dnd_state.target_rank
}

fn setup_current_cell_color(context: &Context, col: i8, row: i8, chess_state: &ChessState) {
    let (w_cells_red, w_cells_green, w_cells_blue) = chess_state.white_cells_color;
    let (b_cells_red, b_cells_green, b_cells_blue) = chess_state.black_cells_color;
    let is_white_cell = (row + col) % 2 == 0;
    if is_white_cell {
        context.set_source_rgb(w_cells_red, w_cells_green, w_cells_blue);
    } else {
        context.set_source_rgb(b_cells_red, b_cells_green, b_cells_blue);
    }
}

fn setup_dnd_highlight_if_matches(
    context: &Context,
    col: i8,
    row: i8,
    chess_state: &ChessState,
    dnd_state: &DndState,
) {
    let (start_cell_red, start_cell_green, start_cell_blue) = chess_state.dnd_start_cell_color;
    let (end_cell_red, end_cell_green, end_cell_blue) = chess_state.dnd_end_cell_color;
    let (cross_cell_red, cross_cell_green, cross_cell_blue) = chess_state.dnd_cross_color;
    if dnd_state.dnd_active {
        if is_dnd_target_cell(col, row, chess_state, dnd_state) {
            context.set_source_rgb(end_cell_red, end_cell_green, end_cell_blue);
        } else if is_dnd_start_cell(col, row, chess_state, dnd_state) {
            context.set_source_rgb(start_cell_red, start_cell_green, start_cell_blue);
        } else if is_dnd_cross_cell(col, row, chess_state, dnd_state) {
            context.set_source_rgb(cross_cell_red, cross_cell_green, cross_cell_blue);
        }
    }
}

fn fill_current_cell_to_setup(context: &Context, col: i8, row: i8, cells_size: f64) {
    let cell_x = cells_size * (0.5 + (col as f64));
    let cell_y = cells_size * (0.5 + (row as f64));
    context.rectangle(cell_x, cell_y, cells_size, cells_size);
    context.fill();
}
