use relm::{Widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use relm_derive::{Msg, widget};

#[allow(dead_code)]
pub struct ChessBoardModel {
    size: i32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
}

pub struct ChessBoardModelBuilder {
    size: i32,
    background_color: (f64, f64, f64),
    white_cells_color: (f64, f64, f64),
    black_cells_color: (f64, f64, f64),
}

#[allow(dead_code)]
impl ChessBoardModelBuilder {
    fn new() -> ChessBoardModelBuilder {
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
        }
    }

    fn set_size(&mut self, size: i32) {
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
    fn model(size: i32) -> ChessBoardModel {
        let mut model_builder = ChessBoardModelBuilder::new();
        model_builder.set_size(size);
        model_builder.build()
    }

    fn update(&mut self, event: ChessBoardMsg) {
        match event {

        }
    }

    fn init_view(&mut self) {
        self.canvas.set_size_request(self.model.size, self.model.size);

        let (bg_red, bg_green, bg_blue) = self.model.background_color;
        let (w_cells_red, w_cells_green, w_cells_blue) = self.model.white_cells_color;
        let (b_cells_red, b_cells_green, b_cells_blue) = self.model.black_cells_color;

        let cells_size = (self.model.size as f64) / 9.0;

        self.canvas.connect_draw(move |_src, context| {
            context.set_source_rgb(bg_red, bg_green, bg_blue);
            context.paint();

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

            Inhibit(true)
        });
    }

    view! {
        #[name="canvas"]
        gtk::DrawingArea {
            
        }
    }
}
