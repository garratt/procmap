extern crate cairo;
extern crate gio;
extern crate gtk;
// Voronoi crates:
// extern crate voronator;
extern crate rand;

// use voronator::CentroidDiagram;
use rand::prelude::*;
use rand::distributions::Uniform;

// poisson disk:
extern crate poisson;
// use rand::FromEntropy;
// use rand::rngs::SmallRng;
use poisson::{Builder, Type, algorithm};

extern crate nalgebra as na;


use gio::prelude::*;
use gtk::prelude::*;
use std::f64::consts::PI;

use gtk::Application;
use gtk::DrawingArea;
// use gtk::{Application, ApplicationWindow, Button};
use cairo::Context;
// use cairo::{Context, FontSlant, FontWeight};

fn my_draw_fn(drawing_area: &DrawingArea, cr: &Context) -> gtk::Inhibit {
    let window_height = drawing_area.get_allocated_height() as f64;
    let window_width = drawing_area.get_allocated_width() as f64;
    cr.scale(window_width, window_height);

    cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
    cr.paint();

    cr.set_line_width(0.01);

    // border
    cr.set_source_rgb(0.0, 0.0, 0.0);
    // cr.rectangle(0.0, 0.0, 0.95, 0.95);
    // cr.stroke();

    // cr.move_to(0.0, 0.0);
    // cr.rel_line_to(0.5, 0.0);
    // cr.rel_line_to(0.0, 0.5);
    // cr.rel_line_to(-0.5, 0.1);
    // cr.close_path();
    // cr.stroke();

    // dimentions are in grid squares
    let grid_dim_x = 10;
    let grid_dim_y = 10;
    // Generate dots for the center of each square:
    let mut points: Vec<(f64, f64)> = vec![(0., 0.); grid_dim_x * grid_dim_y];
    for x in 0..grid_dim_x {
        // want to go from 0->1.0 as center of grid_dim_x boxes,
        // so add 1/2 box width to get to center
        let x_coord = (x as f64 + 0.5) / (grid_dim_x as f64);
        for y in 0..grid_dim_y {
            let y_coord = (y as f64 + 0.5) / (grid_dim_y as f64);
            points[x + y * grid_dim_x] = (x_coord, y_coord);
        }
    }

    // Generate dots randomly:
    let mut rng = rand::thread_rng();
    let range1 = Uniform::new(0., 1.);
    let range2 = Uniform::new(0., 1.);
    let points_v: Vec<(f64, f64)> = (0..(grid_dim_x * grid_dim_y))
        .map(|_| (rng.sample(&range1), rng.sample(&range2)))
        .collect();

    // Generate points with poisson disk sampling:
    let points_p =
    Builder::<_, na::Vector2<f64>>::with_radius(0.05, Type::Normal)
            .build(rng, algorithm::Bridson).generate();

    for p in points {
      cr.arc(p.0, p.1, 0.01, 0.0, PI * 2.);
      cr.fill();
    }

    cr.set_source_rgb(0.50, 0.0, 0.0);
    for p in points_v {
      cr.arc(p.0, p.1, 0.01, 0.0, PI * 2.);
      cr.fill();
    }

    cr.set_source_rgb(0.0, 0.50, 0.0);
    for p in points_p {
      cr.arc(p[0], p[1], 0.01, 0.0, PI * 2.);
      cr.fill();
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
