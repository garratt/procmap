extern crate cairo;
extern crate gio;
extern crate gtk;
// Voronoi crates:
extern crate rand;
extern crate voronator;
// extern crate math;

use rand::distributions::Uniform;
use rand::prelude::*;
use voronator::VoronoiDiagram;
use voronator::delaunator::*;
// use math::round;

// poisson disk:
extern crate poisson;
// use rand::FromEntropy;
// use rand::rngs::SmallRng;
use poisson::{algorithm, Builder, Type};

extern crate nalgebra as na;

use gio::prelude::*;
use gtk::prelude::*;
use std::f64::consts::PI;

use gtk::Application;
use gtk::DrawingArea;
// use gtk::{Application, ApplicationWindow, Button};
use cairo::Context;
// use cairo::{Context, FontSlant, FontWeight};

fn get_poisson_disk_points(radius: f64) -> Vec<(f64, f64)> {
    Builder::<_, na::Vector2<f64>>::with_radius(radius, Type::Normal)
        .build(rand::thread_rng(), algorithm::Bridson)
        .generate() // makes a Vec<Vector2<f64>>
        .into_iter() // start iterating over that ^
        .map(|x| (x[0], x[1])) // convert Vector2<f64> into (f64, f64)
        .collect() // put it back into the Vec
}

// radius is half grid width. width==height for now
fn get_uniform_grid(radius: f64) -> Vec<(f64, f64)> {
    // dimentions are in grid squares
    let grid_dim_x = (1.0 / (radius * 2.)).ceil() as usize;
    let grid_dim_y = (1.0 / (radius * 2.)).ceil() as usize;
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
    points
}

fn get_random_dots(radius: f64) -> Vec<(f64, f64)> {
    // Generate dots randomly:
    let grid_dim_x = (1.0 / (radius * 2.)).ceil() as usize;
    let grid_dim_y = (1.0 / (radius * 2.)).ceil() as usize;
    let mut rng = rand::thread_rng();
    let range1 = Uniform::new(0., 1.);
    let range2 = Uniform::new(0., 1.);
    let points: Vec<(f64, f64)> = (0..(grid_dim_x * grid_dim_y))
        .map(|_| (rng.sample(&range1), rng.sample(&range2)))
        .collect();
    points
}

fn verify_neighbor(cells1 : &Vec<voronator::delaunator::Point>, cells2: &Vec<voronator::delaunator::Point>) -> bool {
   for pt1 in cells1 {
       for pt2 in cells2 {
           if (pt1.x == pt2.x) && (pt1.y == pt2.y) {
               return true;
           }
       }
   }
   false
}

fn is_edge(cell_index : usize, diagram: &voronator::VoronoiDiagram) -> bool {
   if diagram.cells[cell_index].len() != diagram.neighbors[cell_index].len() {
       return true;
   }
   // Check that neighbors are real
   for n in &diagram.neighbors[cell_index] {
       if !verify_neighbor(&diagram.cells[cell_index], &diagram.cells[n.clone()]) {
           return true;
       }
   }
   false
}


fn draw_voronoi(diagram: &voronator::VoronoiDiagram, cr: &Context) {
    for (i, cell) in diagram.cells.iter().enumerate() {
        // if diagram.cells[i].len() == diagram.neighbors[i].len() {
        if is_edge(i, &diagram) {
           cr.set_source_rgb(0., 0., 1.0);
        } else {
           let center = &diagram.sites[i];
           cr.set_source_rgb(center.x, center.y, 0.0);
        // cr.set_source_rgb(0.0, 0.0, 0.0);
        // for pt in &diagram.neighbors[i] {
            // let n_center : &voronator::delaunator::Point = &diagram.sites[pt.clone()];
            // cr.move_to(center.x, center.y);
            // cr.line_to(n_center.x, n_center.y);
            // cr.stroke()
        // }
        }
        if let Some(last) = cell.last() {
            cr.move_to(last.x, last.y);
        }
        for pt in cell {
            cr.line_to(pt.x, pt.y);
        }
            // cr.stroke()
        cr.stroke_preserve();
        cr.fill();
    }
    for p in &diagram.sites {
        cr.arc(p.x, p.y, 0.001, 0.0, PI * 2.);
        cr.fill();
    }
}

fn my_draw_fn(drawing_area: &DrawingArea, cr: &Context) -> gtk::Inhibit {
    let window_height = drawing_area.get_allocated_height() as f64;
    let window_width = drawing_area.get_allocated_width() as f64;
    let grid_size = 0.03;
    cr.scale(window_width, window_height);

    cr.set_source_rgb(250.0 / 255.0, 224.0 / 255.0, 55.0 / 255.0);
    cr.paint();

    cr.set_line_width(0.001);

    // cr.set_source_rgb(0.0, 0.0, 0.0);
    // let points = get_uniform_grid(grid_size);
    // for p in points {
        // cr.arc(p.0, p.1, 0.001, 0.0, PI * 2.);
        // cr.fill();
    // }

    // cr.set_source_rgb(0.50, 0.0, 0.0);
    // let points_v = get_random_dots(grid_size);
    // draw_voronoi(
        // &VoronoiDiagram::from_tuple(&(0., 0.), &(1., 1.), &points_v).unwrap(),
        // &cr,
    // );

    cr.set_source_rgb(0.0, 0.50, 0.0);
    let points_p2 = get_poisson_disk_points(grid_size);
    draw_voronoi(
        &VoronoiDiagram::from_tuple(&(0., 0.), &(1., 1.), &points_p2).unwrap(),
        &cr,
    );

    Inhibit(false)
}

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("Testing Voronoi");
        window.set_default_size(1000, 1000);
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
