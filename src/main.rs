extern crate cairo;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::f64::consts::PI;

use gtk::Application;
use gtk::DrawingArea;
// use gtk::{Application, ApplicationWindow, Button};
use cairo::Context;
// use cairo::{Context, FontSlant, FontWeight};

// fn my_draw_fn(drawing_area: &DrawingArea, cr: &Context) -> dyn Inhibit + 'static {
fn my_draw_fn(_drawing_area: &DrawingArea, cr: &Context) -> gtk::Inhibit {
    cr.scale(100f64, 100f64);

    cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
    cr.paint();

    cr.set_line_width(0.01);

    // border
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.rectangle(0.0, 0.0, 0.95, 0.95);
    cr.stroke();

    cr.move_to(0.0, 0.0);
    cr.rel_line_to(0.5, 0.0);
    cr.rel_line_to(0.0, 0.5);
    cr.rel_line_to(-0.5, 0.1);
    cr.close_path();
    cr.stroke();

    // Draw dots
    for x in 1..10 {
        let x = x as f64 * 0.1;
        for y in 1..10 {
            let y = y as f64 * 0.1;
            cr.arc(x, y, 0.01, 0.0, PI * 2.);
            cr.fill();
        }
    }
    Inhibit(false)
}

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(500, 500);
        let drawing_area = Box::new(DrawingArea::new)();

        drawing_area.connect_draw(my_draw_fn);

        window.add(&drawing_area);

        // let button = Button::with_label("Click me!");
        // button.connect_clicked(|_| {
        // println!("Clicked!");
        // });
        // window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
