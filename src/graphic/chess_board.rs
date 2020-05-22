use relm::{Widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use relm_derive::{Msg, widget};

pub struct ChessBoardModel {
    size: i32,
}

#[derive(Msg)]
pub enum ChessBoardMsg {
    
}

#[widget]
impl Widget for ChessBoard {
    fn model(size: i32) -> ChessBoardModel {
        ChessBoardModel {
            size,
        }
    }

    fn update(&mut self, event: ChessBoardMsg) {
        match event {

        }
    }

    fn init_view(&mut self) {
        self.canvas.set_size_request(self.model.size, self.model.size);
        self.canvas.connect_draw(|_self, context| {
            context.set_source_rgb(0.5, 0.4, 0.9);
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
