use relm::{Widget};
use gtk::prelude::*;
use gtk::{Inhibit};
use relm_derive::{Msg, widget};

pub struct ChessBoardModel {

}

#[derive(Msg)]
pub enum ChessBoardMsg {
    
}

#[widget]
impl Widget for ChessBoard {
    fn model() -> ChessBoardModel {
        ChessBoardModel {}
    }

    fn update(&mut self, event: ChessBoardMsg) {
        match event {

        }
    }

    fn init_view(&mut self) {
        self.canvas.set_size_request(300, 300);
        self.canvas.connect_draw(|_self, context| {
            context.set_source_rgb(0.3, 0.4, 0.9);
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
