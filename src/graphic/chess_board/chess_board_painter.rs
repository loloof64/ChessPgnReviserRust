use failure::Fail;

use cairo::{Context, FontFace, FontWeight, ImageSurface};
use resvg::backend_cairo::render_to_image;
use resvg::usvg::ShapeRendering;
use resvg::{usvg::Tree, FitTo, Options};
use std::collections::HashMap;

use super::chess_board_widget::BlackSide;

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

            if let Some(image) = image_to_render {
                self.images.insert(fen, image);
            }
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

    fn get_image(&self, fen: char) -> Result<ImageSurface, ChessPiecesError> {
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

    pub fn draw_background(&self, context: &Context, background_color: (f64, f64, f64)) {
        let (bg_red, bg_green, bg_blue) = background_color;

        context.set_source_rgb(bg_red, bg_green, bg_blue);
        context.paint();
    }

    pub fn draw_cells(
        &self,
        context: &Context,
        white_cells_color: (f64, f64, f64),
        black_cells_color: (f64, f64, f64),
    ) {
        let (w_cells_red, w_cells_green, w_cells_blue) = white_cells_color;
        let (b_cells_red, b_cells_green, b_cells_blue) = black_cells_color;

        let cells_size = self.cells_size as f64;

        for row in 0..8 {
            for col in 0..8 {
                let is_white_cell = (row + col) % 2 == 0;
                if is_white_cell {
                    context.set_source_rgb(w_cells_red, w_cells_green, w_cells_blue);
                } else {
                    context.set_source_rgb(b_cells_red, b_cells_green, b_cells_blue);
                }

                let cell_x = cells_size * (0.5 + (col as f64));
                let cell_y = cells_size * (0.5 + (row as f64));
                context.rectangle(cell_x, cell_y, cells_size, cells_size);
                context.fill();
            }
        }
    }

    pub fn draw_pieces(&self, context: &Context, position: &str, black_side: BlackSide) {
        let board_value = position.split(" ").take(1).collect::<Vec<_>>()[0];
        let pieces_lines = board_value.split("/").collect::<Vec<_>>();

        let ascii_0 = 48;
        let ascii_9 = 57;
        for (line_index, line) in pieces_lines.iter().enumerate() {
            let line_values = line.chars();
            let mut col_index = 0;

            for value in line_values {
                let value_ascii = value as u8;
                let is_digit_value = value_ascii >= ascii_0 && value_ascii <= ascii_9;

                if !is_digit_value {
                    let col = col_index;
                    let row = line_index;

                    let cells_size = self.cells_size as f64;
                    let x = cells_size
                        * (0.5
                            + if black_side == BlackSide::BlackBottom {
                                7 - col
                            } else {
                                col
                            } as f64);
                    let y = cells_size
                        * (0.5
                            + if black_side == BlackSide::BlackBottom {
                                7 - row
                            } else {
                                row
                            } as f64);

                    self.draw_single_piece(
                        context,
                        self.pieces_images
                            .get_image(value)
                            .expect(format!("could not get image for {}", value).as_str()),
                        x,
                        y,
                    );
                    col_index += 1;
                } else {
                    let holes_count = value_ascii - ascii_0;
                    col_index += holes_count;
                }
            }
        }
    }

    pub fn draw_player_turn(&self, context: &Context, position: &str) {
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

    pub fn draw_coordinates(
        &self,
        context: &Context,
        coordinates_color: (f64, f64, f64),
        black_side: BlackSide,
    ) {
        self.prepare_coordinates_drawing(&context, coordinates_color);
        self.draw_files_coordinates(&context, black_side);
        self.draw_ranks_coordinates(&context, black_side);
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

    fn draw_files_coordinates(&self, context: &Context, black_side: BlackSide) {
        let cells_size = self.cells_size as f64;
        let ascii_uppercase_a = 65u8;

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

    fn draw_ranks_coordinates(&self, context: &Context, black_side: BlackSide) {
        let cells_size = self.cells_size as f64;
        let ascii_1 = 49u8;

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

    fn draw_single_piece(&self, context: &Context, image: ImageSurface, x: f64, y: f64) {
        let origin = 0f64;

        context.save();
        context.translate(x, y);
        context.set_source_surface(&image, origin, origin);
        context.paint();
        context.fill();
        context.restore();
    }
}
