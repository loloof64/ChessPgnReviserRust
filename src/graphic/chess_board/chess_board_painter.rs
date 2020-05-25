
use failure::{Fail};

use resvg::backend_cairo::render_node_to_image;
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
    images: HashMap<(char, u32), ImageSurface>
}

impl ChessPiecesImages {
    fn new() -> Self {
        ChessPiecesImages{ images: HashMap::new() }
    }

    fn build_images(&mut self, cells_size: u32) {
        let default_options = resvg::Options::default();
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
                &default_options.usvg).ok());

            if let Some(Some(image_tree)) = svg_content {
                let piece_image = render_node_to_image(&image_tree.root(), &default_options)
                .expect(format!("failed to build image for {} and size {}", fen, cells_size).as_str());
                self.images.insert((fen, cells_size), piece_image);
            }
            else {
                println!("failed to build image for {} and size {}", fen, cells_size);
            }
        }
    }

    fn get_image(&self, fen: char, size: u32) -> Result<ImageSurface, ChessPiecesError> {
        let image_id = (fen, size);

        match self.images.get(&image_id) {
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
        size: u32,
    ) {
        let (w_cells_red, w_cells_green, w_cells_blue) = white_cells_color;
        let (b_cells_red, b_cells_green, b_cells_blue) = black_cells_color;

        let cells_size = (size as f64) / 9.0;

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
    ) {
        self.draw_single_piece(context,
             self.pieces_images.get_image('N', self.cells_size).expect("could not get image for "), 
             80f64, 200f64);
        self.draw_single_piece(context,
             self.pieces_images.get_image('k', self.cells_size).expect("could not get image for "), 
             200f64, 30f64);
    }

    fn draw_single_piece(
        &self,
        context: &cairo::Context,
        image: ImageSurface,
        x: f64,
        y: f64
    ) {
        let base_size = 45f64;
        let origin = 0f64;

        let scale = (self.cells_size as f64) / base_size;

        context.save();
        context.translate(x, y);
        context.scale(scale, scale);
        context.set_source_surface(&image, origin, origin);
        context.rectangle(origin, origin, base_size, base_size);
        context.fill();
        context.restore();
    }
}