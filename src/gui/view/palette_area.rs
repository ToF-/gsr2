use gtk::Align;
use gtk::cairo::{Context, Format, ImageSurface};
use gtk::prelude::DrawingAreaExt;
use gtk::prelude::DrawingAreaExtManual;
use gtk::prelude::WidgetExt;
use palette_extract::Color;

pub fn make_palette_area(sample: Vec<Color>, width: i32, height: i32) -> gtk::DrawingArea {
    let palette_area = gtk::DrawingArea::new();
    palette_area.set_valign(Align::Center);
    palette_area.set_halign(Align::Center);
    palette_area.set_content_width(width);
    palette_area.set_content_height(height);
    palette_area.set_draw_func(move |_, ctx, _, _| draw_palette(ctx, width, height, &sample));
    palette_area
}
fn draw_palette(ctx: &Context, width: i32, height: i32, sample: &Vec<Color>) {
    const COLOR_MAX: f64 = 9.0;
    let square_size: f64 = height as f64;
    let offset: f64 = (width as f64 - (COLOR_MAX * square_size)) / 2.0;
    let surface =
        ImageSurface::create(Format::ARgb32, width, height).expect("can't create surface");
    let context = Context::new(&surface).expect("can't create context");
    for (i, color) in sample.iter().enumerate() {
        context.set_source_rgb(
            color.r as f64 / 255.0,
            color.g as f64 / 255.0,
            color.b as f64 / 255.0,
        );
        let x = i as f64 * square_size;
        context.rectangle(offset + x, 0.0, square_size, square_size);
        context.fill().expect("can't fill rectangle");
    }
    ctx.set_source_surface(&surface, 0.0, 0.0)
        .expect("can't set source surface");
    ctx.paint().expect("can't paint surface")
}
