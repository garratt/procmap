extern crate gtk;
extern crate cairo;
extern crate gio;

use std::f64::consts::PI;
use gtk::prelude::*;
use gio::prelude::*;

use gtk::DrawingArea;
use gtk::{Application};
// use gtk::{Application, ApplicationWindow, Button};
use cairo::{Context};
// use cairo::{Context, FontSlant, FontWeight};

fn build_ui(application: &gtk::Application) {
    drawable(application, 500, 500, |_, cr| {
        // cr.set_dash(&[3., 2., 1.], 1.);
        // assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

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


        // cr.set_line_width(0.03);

        // draw circle
        // cr.arc(0.5, 0.5, 0.4, 0.0, PI * 2.);
        // cr.stroke();

        // mouth
        // let mouth_top = 0.68;
        // let mouth_width = 0.38;

        // let mouth_dx = 0.10;
        // let mouth_dy = 0.10;

        // cr.move_to(0.50 - mouth_width / 2.0, mouth_top);
        // cr.curve_to(
            // 0.50 - mouth_dx,
            // mouth_top + mouth_dy,
            // 0.50 + mouth_dx,
            // mouth_top + mouth_dy,
            // 0.50 + mouth_width / 2.0,
            // mouth_top,
        // );

        // println!("Extents: {:?}", cr.fill_extents());

        // cr.stroke();

        // let eye_y = 0.38;
        // let eye_dx = 0.15;
        // cr.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, PI * 2.);
        // cr.fill();

        // cr.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, PI * 2.);
        // cr.fill();

        Inhibit(false)
    });
}
fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        build_ui(app);

        // let window = ApplicationWindow::new(app);
        // window.set_title("First GTK+ Program");
        // window.set_default_size(500, 500);
        // let drawing_area = Box::new(DrawingArea::new)();

        // drawing_area.connect_draw(draw_fn);

        // let button = Button::with_label("Click me!");
        // button.connect_clicked(|_| {
            // println!("Clicked!");
        // });
        // window.add(&button);

        // window.show_all();
    });

    application.run(&[]);
}

pub fn drawable<F>(application: &gtk::Application, width: i32, height: i32, draw_fn: F)
where
    F: Fn(&DrawingArea, &Context) -> Inhibit + 'static,
{
    let window = gtk::ApplicationWindow::new(application);
    let drawing_area = Box::new(DrawingArea::new)();

    drawing_area.connect_draw(draw_fn);

    window.set_default_size(width, height);

    window.add(&drawing_area);
    window.show_all();
}
