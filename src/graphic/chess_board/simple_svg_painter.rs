extern crate nsvg;
use cairo::{Context};

pub fn draw_svg(context: &Context, content: String, left: f64, top: f64, scale: f32) {
    let svg = nsvg::parse_str(&content[..], nsvg::Units::Pixel, 96.0).expect("failed to parse content");
    let image = svg.rasterize_to_raw_rgba(scale).expect("failed to rasterize image");
    let (width, height, data) = image;

    context.save();
    context.translate(left, top);

    for y in 0..height {
        for x in 0..width {
            let first_index = (4*(height * y + x)) as usize;
                let red = *data.get(first_index).expect("failed to get red");
                let green = *data.get(first_index+1).expect("failed to get green");
                let blue = *data.get(first_index+2).expect("failed to get blue");
                let alpha = *data.get(first_index+3).expect("failed to get alpha");

                let red = (red as f64) / 255f64;
                let green = (green as f64) / 255f64;
                let blue = (blue as f64) / 255f64;
                let alpha = (alpha as f64) / 255f64;

                context.set_source_rgba(red, green, blue, alpha);
                context.move_to(x as f64, y as f64);
                context.line_to((x+1) as f64, (y+1) as f64);
                context.stroke();
        }
    }

    context.restore();
}
