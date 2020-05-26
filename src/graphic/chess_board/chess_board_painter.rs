
use failure::{Fail};

use resvg::backend_cairo::render_to_image;
use resvg::usvg::ShapeRendering;
use resvg::FitTo;
use cairo::ImageSurface;
use std::collections::HashMap;

#[derive(Debug, Fail)]
pub enum ChessPiecesError {
    #[fail(display="Bad piece fen: {}", fen)]
    BadPieceFenReference {
        fen: char
    }
}

struct ChessPiecesImages {
    images: HashMap<char, ImageSurface>
}

impl ChessPiecesImages {
    fn new() -> Self {
        ChessPiecesImages{ images: HashMap::new() }
    }

    fn build_images(&mut self, cells_size: u32) {
        let base_size = 45f32;
        let scale = cells_size as f32 / base_size;
        let mut options = resvg::Options::default();
        options.usvg.shape_rendering = ShapeRendering::GeometricPrecision;
        options.fit_to = FitTo::Zoom(scale);


        for fen in vec!['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'] {
            let svg_content = match fen {
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
            }.map(|file_content| resvg::usvg::Tree::from_data(
                file_content.as_bytes(), 
                &options.usvg).ok());

            if let Some(Some(image_tree)) = svg_content {
                let piece_image = render_to_image(&image_tree, &options)
                .expect(format!("failed to build image for {} and size {}", fen, cells_size).as_str());
                self.images.insert(fen, piece_image);
            }
            else {
                println!("failed to build image for {} and size {}", fen, cells_size);
            }
        }
    }

    fn get_image(&self, fen: char) -> Result<ImageSurface, ChessPiecesError> {
        match self.images.get(&fen) {
            Some(image) => Ok((image).clone()),
            None => Err(ChessPiecesError::BadPieceFenReference{fen})
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

    pub fn draw_background(
        &self, 
        context: &cairo::Context, 
        background_color: (f64, f64, f64)
    ) {
        let (bg_red, bg_green, bg_blue) = background_color;

        context.set_source_rgb(bg_red, bg_green, bg_blue);
        context.paint();
    }

    pub fn draw_cells(
        &self,
        context: &cairo::Context,
        white_cells_color: (f64, f64, f64),
        black_cells_color: (f64, f64, f64),
    ) {
        let (w_cells_red, w_cells_green, w_cells_blue) = white_cells_color;
        let (b_cells_red, b_cells_green, b_cells_blue) = black_cells_color;

        let cells_size = self.cells_size as f64;

        for row in 0..8 {
            for col in 0..8 {
                let is_white_cell = (row+col) % 2 == 0;
                if is_white_cell {
                    context.set_source_rgb(w_cells_red, w_cells_green, w_cells_blue);
                }
                else {
                    context.set_source_rgb(b_cells_red, b_cells_green, b_cells_blue);
                }

                let cell_x = cells_size * (0.5 + (col as f64));
                let cell_y = cells_size * (0.5 + (row as f64));
                context.rectangle(cell_x, cell_y, cells_size, cells_size);
                context.fill();
            }
        }
    }

    pub fn draw_pieces(
        &self,
        context: &cairo::Context,
        position: &str,
    ) {
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
                    let x = cells_size * (0.5 + (col as f64));
                    let y = cells_size * (0.5 + (row as f64));

                    self.draw_single_piece(
                        context,
                        self.pieces_images.get_image(value)
                        .expect(format!("could not get image for {}", value).as_str()), 
                        x, y);
                        col_index += 1;
                }
                else {
                    let holes_count = value_ascii - ascii_0;
                    col_index += holes_count;
                }

            }
        }
    }

    pub fn draw_player_turn(
        &self,
        context: &cairo::Context,
        position: &str
    ) {
        let turn_str = position.split(" ").skip(1).take(1).collect::<Vec<_>>()[0];
        let is_white_turn = turn_str == "w";

        let cells_size = self.cells_size as f64;
        let location = cells_size * 8.75;
        let radius = cells_size * 0.25;
        let color = if is_white_turn { (1f64, 1f64, 1f64) } else { (0f64, 0f64, 0f64)};

        context.set_source_rgb(color.0, color.1, color.2);
        context.arc(location, location, radius, 0f64, 2f64 * std::f64::consts::PI);
        context.fill();
    }

    fn draw_single_piece(
        &self,
        context: &cairo::Context,
        image: ImageSurface,
        x: f64,
        y: f64
    ) {
        let origin = 0f64;

        context.save();
        context.translate(x, y);
        context.set_source_surface(&image, origin, origin);
        context.paint();
        context.fill();
        context.restore();
    }
}