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

const TIME_STEP: f32 = 0.02; // 20ms
const SPACE_STEP: f32 = 0.02; // 20mm

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    egui: Egui,
    is_simulation_running: bool,
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

    let mut materials = Vec::with_capacity(10);

    for y in 0..10usize {
        materials.push(Vec::with_capacity(10));
        for x in 0..10usize {
            if y == 0 || x == 9 {
                materials[y].push(&COPPER);
            } else if x == 8 {
                materials[y].push(&VOID);
            } else {
                materials[y].push(&AIR);
            }
        }
    }

    let mut heat_simulation = Simulation::from_material_grid(
        materials,
        273.15,
        Parameters {
            time_step: TIME_STEP,
            space_step: SPACE_STEP,
            width: 10,
            height: 10,
        },
    );
    heat_simulation.set_temperature(5273.15, 0, 0);
    heat_simulation.set_temperature(5273.15, 9, 0);

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
        is_simulation_running: false,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    // Simulation
    if model.is_simulation_running {
        // Update simulation
        let start = SystemTime::now();
        model.heat_simulation.update();
        model.performances.update_time.push(start.elapsed().unwrap());

        // Update sim elapsed_time
        model.elapsed_time += Duration::from_secs_f32(TIME_STEP);
    }

    // Update frame time
    model.performances.frame_time.push(update.since_last);

    // GUI
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();
    egui::Window::new("Infos").show(&ctx, |ui| {
        // Graphics Performace
        ui.label(format!("Frame Time: {:.1?}", model.performances.frame_time));
        ui.label(format!(
            "FPS: {:.1?}",
            1.0 / model.performances.frame_time.average.as_secs_f64()
        ));
        ui.separator();
        // Time
        ui.label(format!("Real Time: {:.1?}", update.since_start));
        ui.label(format!("Sim Time: {:.1?}", model.elapsed_time));
        ui.separator();
        // Simulation Performance
        ui.label(format!("Update Time: {:.1?}", model.performances.update_time));
        ui.separator();
        // Controls
        let run_button_text = if model.is_simulation_running { "pause" } else { "run" };
        if ui.button(run_button_text).clicked() {
            model.is_simulation_running = !model.is_simulation_running;
        }
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
