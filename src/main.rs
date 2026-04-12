use std::time::{Duration, SystemTime};

use nannou::prelude::*;
use nannou_egui::{
    self, Egui,
    egui::{self},
};

use crate::{
    material::{AIR, COPPER, VOID},
    simulation::{Parameters, Simulation},
    sliding_average::SlidingAverage,
};

mod array_2d;
mod color_gradient;
mod material;
mod simulation;
mod sliding_average;

const TIME_STEP: f32 = 0.1; // 100ms
const SPACE_STEP: f32 = 0.02; // 20mm

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    elapsed_time: Duration,
    performances: Performances,
    heat_simulation: Simulation,
}

struct Performances {
    frame_time: SlidingAverage<Duration>,
    update_time: SlidingAverage<Duration, 30>,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();

    let materials = vec![
        vec![&COPPER, &COPPER, &COPPER, &COPPER, &COPPER],
        vec![&VOID, &VOID, &VOID, &VOID, &COPPER],
        vec![&AIR, &AIR, &AIR, &VOID, &COPPER],
        vec![&AIR, &VOID, &VOID, &VOID, &COPPER],
        vec![&COPPER, &COPPER, &COPPER, &COPPER, &COPPER],
    ];

    let mut heat_simulation = Simulation::from_material_grid(
        materials,
        273.15,
        Parameters {
            time_step: TIME_STEP,
            space_step: SPACE_STEP,
            width: 5,
            height: 5,
        },
    );
    heat_simulation.set_temperature(7273.15, 0, 0);

    // Egui
    let egui = Egui::from_window(&window);

    // Model
    Model {
        egui,
        elapsed_time: Duration::from_secs(0),
        performances: Performances {
            frame_time: SlidingAverage::new(),
            update_time: SlidingAverage::new(),
        },
        heat_simulation,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // Update simulation
    let start = SystemTime::now();
    model.heat_simulation.update();
    model.performances.update_time.push(start.elapsed().unwrap());

    // Update sim elapsed_time
    model.elapsed_time += Duration::from_secs_f32(TIME_STEP);

    // Update frame time
    model.performances.frame_time.push(update.since_last);

    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Infos").show(&ctx, |ui| {
        ui.label(format!("Frame Time: {:.1?}", model.performances.frame_time));
        ui.label(format!(
            "FPS: {:.1?}",
            1.0 / model.performances.frame_time.average.as_secs_f64()
        ));
        ui.separator();
        ui.label(format!("Real Time: {:.1?}", update.since_start));
        ui.label(format!("Sim Time: {:.1?}", model.elapsed_time));
        ui.separator();
        ui.label(format!("Update Time: {:.1?}", model.performances.update_time));
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.main_window();
    let (width, height) = window.rect().w_h();

    let draw = app.draw();

    draw.background().color(rgb8(55, 55, 55));

    model.heat_simulation.draw(&draw, width, height);

    draw.to_frame(app, &frame).unwrap();

    model.egui.draw_to_frame(&frame).unwrap();
}
