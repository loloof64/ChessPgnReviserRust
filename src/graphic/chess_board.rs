use relm::{Widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use relm_derive::{Msg, widget};

#[allow(dead_code)]
pub struct ChessBoardModel {
    size: i32,
    background_color: (f64, f64, f64)
}

pub struct ChessBoardModelBuilder {
    size: i32,
    background_color: (f64, f64, f64)
}

#[allow(dead_code)]
impl ChessBoardModelBuilder {
    fn new() -> ChessBoardModelBuilder {
        ChessBoardModelBuilder {
            size: 300,
            background_color: (0.5, 0.4, 0.9)
        }
    }

    fn build(self) -> ChessBoardModel {
        ChessBoardModel {
            size: self.size,
            background_color: self.background_color,
        }
    }

    fn set_size(&mut self, size: i32) {
        self.size = size;
    }

    fn set_background_color(&mut self, background_color: (f64, f64, f64)) {
        self.background_color = background_color;
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
        self.canvas.connect_draw(move |_src, context| {
            context.set_source_rgb(bg_red, bg_green, bg_blue);
            context.paint();
            Inhibit(true)
        });
    }

    view! {
        #[name="canvas"]
        gtk::DrawingArea {
            
        }
    }
}
